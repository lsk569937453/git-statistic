import { Textarea } from "@/components/ui/textarea"
import { useEffect, useState, useRef } from "react"
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button"
import { useToast } from "@/components/ui/use-toast"
import { useTranslation, Trans } from "react-i18next"; import Select from 'react-select'

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
import makeAnimated from 'react-select/animated';

import { ScrollArea } from "@/components/ui/scroll-area"
import { info } from "console";
const animatedComponents = makeAnimated();

export function LineInfoPage() {
    const { t, i18n } = useTranslation();
    const chartRef = useRef<any>(null); // Create a reference for the chart instance

    const [currentInput, setCurrentInput] = useState();
    const { toast } = useToast()
    const [totalLines, setTotalLines] = useState<any>();
    const [lineData, setLineData] = useState<any>([]);
    const [dirsLineData, setDirsLineData] = useState<any>([]);
    const [totalDirsLineData, setTotalDirsLineData] = useState<any>([]);
    const [dirOptions, setDirOptions] = useState<any>([]);
    useEffect(() => {
        loadData();
    }, [])

    const options = [
        { value: 'chocolatessssssssssssssssssssssssssssssssss', label: 'chocolatessssssssssssssssssssssssssssssssss' },
        { value: 'strawberry', label: 'Strawberry' },
        { value: 'vanilla', label: 'Vanilla' },
        { value: 'chocolatessssssssssssssssssssssssssssssssss2', label: 'chocolatessssssssssssssssssssssssssssssssss2' },
        { value: 'chocolatessssssssssssssssssssssssssssssssss3', label: 'chocolatessssssssssssssssssssssssssssssssss3' },
        { value: 'chocolatessssssssssssssssssssssssssssssssss4', label: 'chocolatessssssssssssssssssssssssssssssssss4' }


    ];
    useEffect(() => {
        if (chartRef.current) {
            const chartInstance = chartRef.current.getEchartsInstance();
            chartInstance.setOption(optionsForLocByDirs(), {
                replaceMerge: ['series'], // Use this option correctly
            });
        }
    }, [dirsLineData]); // Trigger when dirsLineData changes

    const loadData = async () => {

        const { response_code, response_msg } = JSON.parse(await invoke("get_line_info"));
        console.log(response_code);
        console.log(response_msg);

        if (response_code === 0) {
            const { line_statistic_data, line_statistic_total_count, dir_loc_info
            } = response_msg;
            let lineData = JSON.parse(line_statistic_data);
            const result = lineData.map((item: any) => [item.date, item.count]);

            setLineData(result);
            let dirLocInfo = JSON.parse(dir_loc_info);
            let dirOptions = dirLocInfo.map((item: any) => ({ value: item.dir_name, label: item.dir_name }));

            setDirOptions(dirOptions);
            setTotalDirsLineData(dirLocInfo);
            setTotalLines(line_statistic_total_count);
        }
    }
    const optionsForLoc = () => {
        if (!lineData || lineData.length === 0) {
            return {

                xAxis: {
                    type: 'category',
                    data: []
                },
                yAxis: {
                    type: 'value',
                },
                series: []
            };
        }
        return {
            title: {
                text: t("linePage.linesOfCodeText"),
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
                    type: 'time',
                    // boundaryGap: false,

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
    const optionsForLocByDirs = () => {
        if (!dirsLineData || dirsLineData.length === 0) {
            return {

                xAxis: {
                    type: 'category',
                    data: []
                },
                yAxis: {
                    type: 'value',
                },
                series: []
            };
        }
        return {
            // title: {
            //     text: "Line Of Code By Directories",
            //     left: 'center',    // Centers the title horizontally
            //     bottom: 10,
            // },
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
                data: dirsLineData.map((task: any) => task.dir_name),
                top: '10%',     // Adjusts the position from the top

            },

            grid: {
                left: '3%',
                right: '4%',
                bottom: '3%',
                containLabel: true
            },
            xAxis: [
                {
                    type: 'time',
                    // boundaryGap: false,

                }
            ],

            series:
                dirsLineData.map((task: any) => ({
                    name: task.dir_name,  // Sync task name for each series
                    data: task.data,      // Corresponding logs data
                    type: 'line',                // Type of chart (bar in this case)
                    emphasis: {
                        focus: 'series'
                    },
                    showSymbol: false,
                }),),
        };
    }
    const handleOnChange = async (value: any) => {
        if (value.length === 0) {
            console.log(value);
            setDirsLineData([]);
            return;
        }
        const valueSet = new Set(value.map((item: any) => item.value));
        const filteredDirs = totalDirsLineData.filter((item: any) => valueSet.has(item.dir_name));
        console.log(filteredDirs);
        setDirsLineData(filteredDirs);

    }
    return (
        <ScrollArea className="h-full">
            <div className="flex flex-col">
                <Card className="pt-4">
                    <CardContent className="flex flex-col gap-5 text-right">
                        <div className="flex flex-row gap-10">
                            <p className="basis-2/12 text-lg font-bold">{t("linePage.totalLinesText")}</p>
                            <p className="text-lg">{totalLines}</p>
                        </div>

                        <Separator />
                        <div className="basis-1/2 bg-white	rounded-lg p-4">
                            <ReactECharts option={optionsForLoc()} />
                        </div>
                        <Separator />
                        <span className="basis-1/2 text-lg font-bold text-center">Line Of Code By Directories</span>

                        <span className="basis-1/2 text-lg font-bold text-left">Please Select the Directories</span>
                        <div className="px-16">
                            <Select
                                closeMenuOnSelect={false}
                                components={animatedComponents}
                                defaultValue={dirOptions ? dirOptions[0] : null}
                                isMulti
                                options={dirOptions}

                                isSearchable={true}
                                onChange={handleOnChange}
                            />
                        </div>
                        <div className="basis-1/2 bg-white	rounded-lg p-4">
                            <ReactECharts option={optionsForLocByDirs()}
                                ref={chartRef} />

                        </div>
                    </CardContent>

                </Card>
            </div>
        </ScrollArea>

    );
}