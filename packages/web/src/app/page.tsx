"use client";

import Image from "next/image";
import { useEffect, useMemo, useState } from "react";
import {
  GrpcWebImpl,
  GasClientImpl,
  Network,
  BlockUpdate,
} from "../../packages/proto/gas";
import { AreaChart, Card, Title } from "@tremor/react";
import Tooltip from "@/components/tooltip";

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
        <Title>Newsletter revenue over time (USD)</Title>
        <AreaChart
          className="h-[70vh] w-full mt-4"
          data={blocks}
          index="blockNumber"
          yAxisWidth={50}
          categories={["gasFee"]}
          colors={["indigo"]}
          valueFormatter={(v) => v.toFixed(2) + " wei"}
          customTooltip={Tooltip}
        />
      </Card>
    </main>
  );
}
