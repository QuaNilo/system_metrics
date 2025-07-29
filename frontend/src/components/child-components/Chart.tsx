import {
  LineChart, Line, XAxis, YAxis, Tooltip, CartesianGrid, Legend, ResponsiveContainer,
} from "recharts";

interface GenericChartProps<T> {
  data: T[];
  xAxisKey: keyof T;
  dataKeys: (keyof T)[];
  chartTitle?: string;
  colors?: string[];
}

export function Chart<T>({
  data,
  xAxisKey,
  dataKeys,
  chartTitle,
  colors = [
  "#8884d8",
  "#82ca9d",
  "#ffc658",
  "#ff7300",
  "#a4de6c",
  "#ffb6c1",
  "#6a5acd",
  "#20b2aa",
  "#ff6347",
  "#7b68ee",
  "#3cb371",
  "#da70d6",
  "#4682b4",
  "#cd5c5c",
  "#40e0d0",
],
}: GenericChartProps<T>) {
  return (
    <div style={{ width: "100%", height: 300 }}>
      {chartTitle && <h3>{chartTitle}</h3>}
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis
            dataKey={String(xAxisKey)}
            tickFormatter={(tick) => {
              // Format timestamp if it's ISO string
              if (typeof tick === "string") {
                return new Date(tick).toLocaleTimeString();
              }
              return String(tick);
            }}
          />
          <YAxis />
          <Tooltip
            labelFormatter={(label) => {
              if (typeof label === "string") {
                return new Date(label).toLocaleString();
              }
              return label;
            }}
          />
          <Legend />
          {dataKeys.map((key, index) => (
            <Line
              key={String(key)}
              type="monotone"
              dataKey={String(key)}
              stroke={colors[index % colors.length]}
              dot={false}
            />
          ))}
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}

export default Chart;