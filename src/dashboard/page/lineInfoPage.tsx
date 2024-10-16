import { Textarea } from "@/components/ui/textarea"
import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button"
import { useToast } from "@/components/ui/use-toast"
import { useTranslation, Trans } from "react-i18next";
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import { set } from "date-fns"; import ReactECharts from 'echarts-for-react';

import { Separator } from "@/components/ui/separator"
import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
import { ScrollArea } from "@/components/ui/scroll-area"

export function LineInfoPage() {
    const { t, i18n } = useTranslation();

    const [currentInput, setCurrentInput] = useState();
    const { toast } = useToast()
    const [totalLines, setTotalLines] = useState<any>();
    const [lineData, setLineData] = useState<any>([]);

    useEffect(() => {
        loadData();
    }, [])
    const loadData = async () => {

        const { response_code, response_msg } = JSON.parse(await invoke("get_line_info"));
        console.log(response_code);
        console.log(response_msg);

        if (response_code === 0) {
            const { line_statistic_data, line_statistic_total_count
            } = response_msg;
            let lineData = JSON.parse(line_statistic_data);
            console.log(lineData);
            const result = lineData.map((item: any) => [item.date, item.count]);

            setLineData(result);

            setTotalLines(line_statistic_total_count);
        }
    }
    const optionsForDataPerMinuteGroupByTaskId = () => {
        if (!lineData || lineData.length === 0) {
            return {

                xAxis: {
                    type: 'category',
                    data: []  // Empty x-axis data
                },
                yAxis: {
                    // name: '同步sql条数',
                    type: 'value',
                },
                series: []  // No series data
            };
        }
        return {
            title: {
                text: 'Lines Of Code'
            },
            tooltip: {
                trigger: 'axis',
                axisPointer: {
                    type: 'cross',
                    label: {
                        backgroundColor: '#6a7985'
                    }
                }
            },
            legend: {
                data: "sss",
            },

            grid: {
                left: '3%',
                right: '4%',
                bottom: '3%',
                containLabel: true
            },
            xAxis: [
                {
                    type: 'time', boundaryGap: false,

                }
            ],

            series: {
                // name: "task.sync_task_name",  // Sync task name for each series
                data: lineData,      // Corresponding logs data
                type: 'line',                // Type of chart (bar in this case)
                emphasis: {
                    focus: 'series'
                },
                showSymbol: false,
            }
        };
    }
    return (
        <ScrollArea className="h-full">
            <div className="flex flex-col">
                <Card className="pt-4">
                    <CardContent className="flex flex-col gap-5 text-right">
                        <div className="flex flex-row gap-10">
                            <p className="basis-2/12 text-lg font-bold">Total Lines:</p>
                            <p className="text-lg">{totalLines}</p>
                        </div>

                        <Separator />
                        <div className="basis-1/2 bg-white	rounded-lg p-4">
                            <ReactECharts option={optionsForDataPerMinuteGroupByTaskId()} />

                        </div>
                    </CardContent>

                </Card>
            </div>
        </ScrollArea>

    );
}