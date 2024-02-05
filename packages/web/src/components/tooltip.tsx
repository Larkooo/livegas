const Tooltip = (props: any) => {
  const { payload, active } = props;
  if (!active || !payload) return null;
  return (
    <div className="w-56 rounded-tremor-default text-tremor-default bg-tremor-background p-2 shadow-tremor-dropdown border border-tremor-border">
      {payload.map((category: {
        color: string;
        dataKey: string;
        value: number;
      }, idx: number) => (
        <div key={idx} className="flex flex-1 space-x-2.5">
          <div
            className={`w-1 flex flex-col bg-${category.color}-500 rounded`}
          />
          <div className="space-y-1">
            <p className="text-tremor-content">{category.dataKey}</p>
            <p className="font-medium text-tremor-content-emphasis">
              {category.value} gwei
            </p>
          </div>
        </div>
      ))}
    </div>
  );
};

export default Tooltip;
