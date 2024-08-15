"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { sha256 } from 'js-sha256'; // Use js-sha256 to calculate the seed
import { createHash, randomBytes } from "crypto";
import { bigint, z } from "zod";
import bs58 from "bs58"

import { Button } from "@/components/ui/button";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { Connection, Transaction, TransactionInstruction, PublicKey, Keypair, sendAndConfirmRawTransaction } from "@solana/web3.js";

const formSchema = z.object({
    name: z.string().min(2, {
        message: "Name must be at least 2 characters.",
    }),
    info: z.string().min(5, {
        message: "Info must be at least 5 characters.",
    }),
    price: z
        .string()
        .min(1, {
            message: "Price is required.",
        })
        .transform((val) => parseInt(val))
        .refine((val) => !isNaN(val) && val >= 0, {
            message: "Price must be a valid number and at least 0.",
        }),
});
const MY_PROGRAM_ID = "4Bprhf44eKn2hm8WiHZZnbHbvUjeb9NKmrseBtYKY8te"
const SystemRent = "SysvarRent111111111111111111111111111111111";
const SystemProgram = "11111111111111111111111111111111";
const COMPANY_PUBLIC_KEY = "2PUXaUE33wcWnAE8y9sEj17SJgDdMEyF3K4wsM7Hdue3"
const COMPANY_PRIVATE_KEY = "2QAXzNw4bvrBvFYBWoorBekKXTHdUo9zKubnRwpZ1sQzkj43ao1gwFfpPpy44ckJvgWeGUfgA2mNXUpjgUTs5hUP"

function convertNameToHash8Bytes(name: string): Buffer {
    const hash = createHash('sha256').update(`global:${name}`).digest();
    return hash.subarray(0, 8); // Take the first 8 bytes
}

function encode(str: string) {
    const encoder = new TextEncoder();
    let buf = encoder.encode(str);
    for (let i = 0; i < buf.length; i++) {
        buf[i]++;
    }
    return buf;
}

function generateRandomSeed(): bigint {
    // Generate 8 random bytes directly
    const random = randomBytes(8);

    // Convert the random bytes to a BigInt
    let seed = BigInt(0);
    for (let i = 0; i < random.length; i++) {
        seed = seed * BigInt(256) + BigInt(random[i]);
    }

    return seed;
}
function bigintToBytes(bigint: bigint, byteLength: number): Uint8Array {
    // Create a Buffer with the required length
    const buffer = Buffer.alloc(byteLength);

    // Fill the buffer with the BigInt value
    for (let i = 0; i < byteLength; i++) {
        buffer[byteLength - 1 - i] = Number(bigint >> BigInt(i * 8)) & 0xff;
    }

    return new Uint8Array(buffer);
}

export async function initGymClass(
    connection: Connection,
    companyPublicKey: PublicKey,
    trainerPublicKey: PublicKey,
    name: string,
    info: string,
    price: number
) {
    const programId = new PublicKey(MY_PROGRAM_ID);
    const companyKeypair = Keypair.fromSecretKey(Uint8Array.from(bs58.decode(COMPANY_PRIVATE_KEY)));
    // Constants and assumptions
    const SEED_PREFIX = "gymclass";

    // Ensure that name and info fit into the required byte arrays
    const nameBytes = encode(name);
    const infoBytes = encode(info);

    if (nameBytes.length > 32 || infoBytes.length > 256) {
        throw new Error("Name or info exceeds maximum length.");
    }

    const nameArray = new Uint8Array(32);
    nameArray.set(nameBytes);

    const infoArray = new Uint8Array(256);
    infoArray.set(infoBytes);

    // Generate the seed_sha256 value
    const seedSha256 = generateRandomSeed();

    const [gymClassAddress] = await PublicKey.findProgramAddressSync(
        [
            companyPublicKey.toBuffer(),
            Buffer.from(SEED_PREFIX),
            bigintToBytes(seedSha256, 8),
        ],
        programId
    );
    console.log(`seed ${seedSha256}`);
    console.log(`gym class address: ${gymClassAddress}`);

    // Create the instruction
    const instruction = new TransactionInstruction({
        keys: [
            { pubkey: companyPublicKey, isSigner: true, isWritable: true },
            { pubkey: trainerPublicKey, isSigner: true, isWritable: true },
            { pubkey: gymClassAddress, isSigner: false, isWritable: true },
            { pubkey: new PublicKey(SystemRent), isSigner: false, isWritable: false },
            { pubkey: new PublicKey(SystemProgram), isSigner: false, isWritable: false },
        ],
        programId,
        data: Buffer.concat([
            convertNameToHash8Bytes("init_gymclass"), // Assuming 0 is the instruction identifier for init_gymclass
            nameArray,
            infoArray,
            Buffer.from(new Uint8Array(new BigUint64Array([BigInt(price)]).buffer)),
            bigintToBytes(seedSha256, 8),
        ]),
    });

    // Send the transaction
    const transaction = new Transaction().add(instruction);
    transaction.feePayer = companyKeypair.publicKey;
    try {
        transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    } catch (error) {
        console.error("Cannot get blockhash", error);
    }
    transaction.sign(companyKeypair);

    return transaction;
}

export default function TrainerForm() {
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: "",
            info: "",
            price: 0,
        },
    });


    const { connection }: { connection: Connection } = useConnection();
    const { publicKey, signTransaction } = useWallet();

    async function onSubmit(values: z.infer<typeof formSchema>) {
        if (!publicKey || !signTransaction) {
            console.error("Wallet not connected");
            return;
        }

        const trainerPublicKey = publicKey; // Assuming the trainer is the currently connected wallet
        const companyPublicKey = new PublicKey(COMPANY_PUBLIC_KEY)

        const name = values.name;
        const info = values.info;
        const price = values.price; // 1 SOL, in lamports (1 SOL = 1,000,000,000 lamports)

        try {
            const transaction = await initGymClass(
                connection,
                companyPublicKey,
                trainerPublicKey,
                name,
                info,
                price
            );

            const signedTransaction = await signTransaction(transaction);
            const signature = await sendAndConfirmRawTransaction(connection, signedTransaction.serialize());

            console.log("Transaction signature:", signature);
        } catch (error) {
            console.error("Transaction failed", error);
        }
    }

    return (
        <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
                <FormField
                    control={form.control}
                    name="name"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Class name</FormLabel>
                            <FormControl>
                                <Input placeholder="shadcn" {...field} />
                            </FormControl>
                            <FormDescription>
                                This is your gym class name.
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <FormField
                    control={form.control}
                    name="info"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Info</FormLabel>
                            <FormControl>
                                <Input placeholder="info" {...field} />
                            </FormControl>
                            <FormDescription>
                                This is your gym class info.
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="price"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Price</FormLabel>
                            <FormControl>
                                <Input
                                    placeholder="price"
                                    type="number"
                                    {...field}
                                />
                            </FormControl>
                            <FormDescription>
                                This is your gym class price.
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button type="submit">Add class</Button>
            </form>
        </Form>
    );
}
