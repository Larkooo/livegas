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
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Subscription } from "rxjs";

export default function Home() {
  const gas = useMemo(
    () => {
      let grpcWeb = new GrpcWebImpl("http://localhost:8081", { debug: true });
      return new GasClientImpl(grpcWeb);
    },
    []
  );

  const [subscription, setSubscription] = useState<Subscription>();
  const [blocks, setBlocks] = useState<BlockUpdate[]>([]);
  const [maxBlocks, setMaxBlocks] = useState(25);

  useEffect(() => {
    // subscribne to new blocks
    const startSubscription = () => {
      // subscribe to new blocks
      const sub = gas.Subscribe({
        network: Network.ETH_MAINNET,
      });

      let subscription = sub.subscribe({
        next: (res) => {
          // add new block to our list
          setBlocks((prev) => [...prev, res]);
        },
        error: (err) => {
          console.error(err);
        },
        complete: () => {
          console.log("complete");
        },
      });

      setSubscription(subscription);
    };

    gas
      .Blocks({
        network: Network.ETH_MAINNET,
        startBlock: maxBlocks,
        endBlock: 0,
      })
      .then((res) => {
        // we set our fetched blocks
        setBlocks(res.blockUpdates);
        // start subscription if not already started
        if (!subscription) {
          startSubscription();
        }
      });
  }, [gas, maxBlocks, subscription]);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <Card>
        <Pill className="absolute top-[10%] right-[10%] w-20 text-sm font-bold">
          LIVE{" "}
          <div className="flex place-self-center rounded-full w-2 h-2 bg-red-500 animate-pulse"></div>
        </Pill>
        <Tabs
          onValueChange={(v) => setMaxBlocks(parseInt(v))}
          defaultValue="25"
          className="w-[400px]"
        >
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="250">250</TabsTrigger>
            <TabsTrigger value="100">100</TabsTrigger>
            <TabsTrigger value="25">25</TabsTrigger>
          </TabsList>
        </Tabs>
        <AreaChart
          className="h-[80vh] w-full mt-4"
          data={blocks}
          index="blockNumber"
          categories={["gasFee"]}
          colors={["indigo"]}
          valueFormatter={(v) => v.toFixed(2) + " gwei"}
          customTooltip={Tooltip}
        ></AreaChart>
      </Card>
    </main>
  );
}
