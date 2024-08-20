"use client";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { useEffect, useMemo, useState } from "react";
import { getAccountRole } from "@/lib/utils";
import UserAccountView from "./UserAccountView";
import { Button } from "@/components/ui/button";

export default function Page() {
    const [role, setRole] = useState("no account");
    const { publicKey } = useWallet();
    const { connection } = useConnection();

    useEffect(() => {
        getAccountRole(publicKey).then((newRole) => {
            setRole(newRole);
        })
    }, [publicKey]);

    if (role === "no account") {
        return (
            <div>
                <div>No account create one?</div>
                <a href="/user/create-account">
                    <Button>Create Account</Button>
                </a>
            </div>
        )
    }
    return (
        publicKey ? <UserAccountView userPubkey={publicKey.toString()} /> : <div>Loading....</div>
    );
}