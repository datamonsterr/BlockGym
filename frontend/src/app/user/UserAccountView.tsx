"use client";
import { UserData } from "@/lib/models";
import { useEffect, useMemo, useState } from "react";

export default function UserAccountView({ userPubkey }: {
    userPubkey: string | null
}) {
    const [data, setData] = useState<UserData>();
    useMemo(() => {
        async function fetchData(public_key: string) {
            const res = await fetch(`http://localhost:8000/get-account-data?public_key=${public_key}`);
            const json = await res.json();
            setData(json.data);
        }
        fetchData(userPubkey ?? "");
    }, [userPubkey])

    return (
        <div>
            <div>{data?.name}</div>
            <div>{data?.role}</div>
            <div>{data?.gender}</div>
            <div>{data?.age}</div>
            <div>{data?.phone}</div>
            <div>{data?.email}</div>
            <div>{data?.info}</div>
        </div>
    );
}