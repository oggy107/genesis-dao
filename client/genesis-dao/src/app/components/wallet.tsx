import React from "react";
import { useAccount, useIsMounted } from "../hooks";
import { ConnectButton } from "./connectButton";

export function WalletData() {
    const mounted = useIsMounted();
    const account = useAccount();

    return (
        <>
            {mounted && account ? (
                <div>
                    <div>{account.displayName}</div>
                </div>
            ) : (
                <ConnectButton label="Connect Wallet" />
            )}
        </>
    );
}
