"use client";

import "./globals.css";
import { cn } from "@/lib/utils";
import {
    NavigationMenu,
    NavigationMenuItem,
    NavigationMenuList,
    navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import React, { useMemo, useState, useEffect } from "react";
import {
    ConnectionProvider,
    WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { clusterApiUrl } from "@solana/web3.js";

// Import CSS for WalletMultiButton (prefer `import` over `require`)
import "@solana/wallet-adapter-react-ui/styles.css";

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const [isMounted, setIsMounted] = useState(false);

    useEffect(() => {
        setIsMounted(true);
    }, []);
    const network = WalletAdapterNetwork.Devnet;
    const endpoint = useMemo(() => clusterApiUrl(network), [network]);

    const wallets = useMemo(
        () => [
            // Add your wallet adapters here, e.g.:
        ],
        [network]
    );

    return (
        <html lang="en" className="dark">
            <body className="w-screen h-fit">
                {
                    isMounted ?
                        <ConnectionProvider endpoint={endpoint}>
                            <WalletProvider wallets={wallets} autoConnect>
                                <WalletModalProvider>
                                    <NavigationMenu className="px-10 bg-transparent">
                                        <NavigationMenuList>
                                            <div className="w-screen px-20 py-5 flex justify-between">
                                                <NavigationMenuItem>
                                                    <a
                                                        href="/"
                                                        className={cn(
                                                            navigationMenuTriggerStyle(),
                                                            " text-2xl rounded-none py-8 bg-inherit"
                                                        )}
                                                    >
                                                        Home
                                                    </a>
                                                </NavigationMenuItem>
                                                <NavigationMenuItem>
                                                    <WalletMultiButton />
                                                </NavigationMenuItem>
                                            </div>
                                        </NavigationMenuList>
                                    </NavigationMenu>
                                    <div id="container" className="w-full h-fit">
                                        {children}
                                    </div>
                                </WalletModalProvider>
                            </WalletProvider>
                        </ConnectionProvider>

                        : null
                }
            </body>
        </html>
    );
}