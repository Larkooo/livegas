"use client";

import Image from "next/image";
import { AreaChart } from "@tremor/react";
import { useEffect, useMemo, useState } from "react";
import {
  GrpcWebImpl,
  GasClientImpl,
  Network,
  BlockUpdate,
} from "../../packages/proto/gas";

export default function Home() {
  const client = useMemo(
    () => new GrpcWebImpl("http://localhost:8081", { debug: true }),
    []
  );
  const gas = useMemo(() => new GasClientImpl(client), [client]);

  useEffect(() => {
    // subscribne to new blocks
    const startSubscription = () => {
      // subscribe to new blocks
      const sub = gas.Subscribe({
        network: Network.ETH_MAINNET,
      });

      sub.subscribe({
        next: (res) => {
          console.log(res);
        },
        error: (err) => {
          console.error(err);
        },
        complete: () => {
          console.log("complete");
        },
      });
    };

    // fetch last 25 blocks
    gas
      .Blocks({
        network: Network.ETH_MAINNET,
        startBlock: 25,
        endBlock: 0,
      })
      .then((res) => {
        console.log(res);
      });
  }, [gas]);


  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      mimi
    </main>
  );
}
