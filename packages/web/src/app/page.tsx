"use client";

import Image from "next/image";
import { useEffect, useMemo, useState } from "react";
import {
  GrpcWebImpl,
  GasClientImpl,
  Network,
  BlockUpdate,
} from "../../packages/proto/gas";
import { AreaChart, Badge, BadgeDelta, Card, Title } from "@tremor/react";
import Tooltip from "@/components/tooltip";
import Pill from "@/components/pill";

export default function Home() {
  const client = useMemo(
    () => new GrpcWebImpl("http://localhost:8081", { debug: true }),
    []
  );
  const gas = useMemo(() => new GasClientImpl(client), [client]);

  const [blocks, setBlocks] = useState<BlockUpdate[]>([]);

  useEffect(() => {
    // subscribne to new blocks
    const startSubscription = () => {
      // subscribe to new blocks
      const sub = gas.Subscribe({
        network: Network.ETH_MAINNET,
      });

      sub.subscribe({
        next: (res) => {
          setBlocks((prev) => [...prev, res]);
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
        // we set our fetched blocks
        setBlocks(res.blockUpdates);
        // and start our subscription
        startSubscription();
      });
  }, [gas]);

  useEffect(() => {
    console.log(blocks);
  }, [blocks]);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <Card>
        <Pill className="absolute top-[10%] right-[10%] w-20 text-sm font-bold">
          LIVE <div className="flex place-self-center rounded-full w-2 h-2 bg-red-500 animate-pulse"></div>
        </Pill>
        <AreaChart
          className="h-[80vh] w-full mt-4"
          data={blocks}
          index="blockNumber"
          yAxisWidth={50}
          categories={["gasFee"]}
          colors={["indigo"]}
          valueFormatter={(v) => v.toFixed(2) + " gwei"}
          customTooltip={Tooltip}
        >
          
        </AreaChart>
      </Card>
    </main>
  );
}
