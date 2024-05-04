import React from "react";
import { setAllowed } from "@stellar/freighter-api";

export interface ConnectButtonProps {
    label: string;
    isHigher?: boolean;
}

export function ConnectButton({ label, isHigher }: ConnectButtonProps) {
    return <button onClick={setAllowed}>{label}</button>;
}
