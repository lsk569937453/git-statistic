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
    const [dirsLineData, setDirsLineData] = useState<any>([]);
    const [totalDirsLineData, setTotalDirsLineData] = useState<any>([]);
    const [selectDirOptions, setSelectDirOptions] = useState<any>([]);
    const [currentSelectDirs, setCurrentSelectDirs] = useState<any>([]);
    useEffect(() => {
        loadData();
    }, [])

    useEffect(() => {
        if (chartRef.current) {
            const chartInstance = chartRef.current.getEchartsInstance();
            chartInstance.setOption(optionsForLocByDirs(), {
                replaceMerge: ['series'], // Use this option correctly
            });
        }
    }, [dirsLineData]);

    const loadData = async () => {

        const { response_code, response_msg } = JSON.parse(await invoke("get_line_info"));
        console.log(response_code);
        console.log(response_msg);

        if (response_code === 0) {
            const { line_statistic_total_count, dir_loc_info, dirs_for_line_info
            } = response_msg;
            let dirLocInfo = JSON.parse(dir_loc_info);
            console.log(dirLocInfo);
            let selectDirOptions = dirLocInfo.map((item: any) => ({ value: item.dir_name, label: item.dir_name }));
            selectDirOptions.sort((a: any, b: any) => {
                const countSlashes = (str: any) => (str.match(/\//g) || []).length;
                const slashCountDiff = countSlashes(a.value) - countSlashes(b.value);

                if (slashCountDiff !== 0) {
                    return slashCountDiff;
                }
                const valueA = a.value.replace(/\//g, '');
                const valueB = b.value.replace(/\//g, '');

                return valueA.localeCompare(valueB);
            });
            let dirsForSelect = JSON.parse(dirs_for_line_info).map((item: any) => ({ value: item, label: item }));
            let dirsForSelectSet = new Set(dirsForSelect.map((item: any) => item.value));
            console.log(dirsForSelect);
            setCurrentSelectDirs(dirsForSelect);
            // Set the selectDirOptions to the sorted options
            setSelectDirOptions(selectDirOptions);
            // Set the totalDirsLineData to the dirLocInfo
            setTotalDirsLineData(dirLocInfo);
            // Set the totalLines to the line_statistic_total_count
            setTotalLines(line_statistic_total_count);

            // Filter the dirsLineData to only include the root directory
            const filteredDirs = dirLocInfo.filter((item: any) => dirsForSelectSet.has(item.dir_name));
            console.log(filteredDirs);
            // Set the dirsLineData to the filteredDirs
            setDirsLineData(filteredDirs);
        }
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
        console.log(value);
        const valueArray = value.map((item: any) => item.value);
        const { response_code, response_msg } = JSON.parse(await invoke("save_dirs_for_line_info", { dirs: valueArray }));

        if (value.length === 0) {
            console.log(value);
            setDirsLineData([]);
            setCurrentSelectDirs([]);
            return;
        }
        const valueSet = new Set(value.map((item: any) => item.value));
        const filteredDirs = totalDirsLineData.filter((item: any) => valueSet.has(item.dir_name));
        console.log(filteredDirs);
        setDirsLineData(filteredDirs);
        setCurrentSelectDirs(value);

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
                        <div className="flex flex-col gap-4">
                            <div className="basis-1/2 text-lg font-bold text-center">Line Of Code By Directories</div>
                            <div className="grid grid-cols-12 gap-4 px-16  border-dashed border-2 border-indigo-600 bg-gray-200">
                                <p className="text-right col-start-1 col-end-2 text-red-500 font-bold">"/":</p>
                                <p className="text-left col-start-2 col-end-10">means the root directory</p>
                                <p className="text-right col-start-1 col-end-2 text-red-500 font-bold" >"/src":</p>
                                <p className="text-left col-start-2 col-end-10">means the src directory in the root directory</p>
                            </div>
                        </div>
                        <span className="basis-1/2 text-lg font-bold text-left">Choose Directories or Search Keywords to Select</span>
                        <div className="px-16">
                            <Select
                                closeMenuOnSelect={false}
                                components={animatedComponents}
                                value={currentSelectDirs}
                                isMulti
                                options={selectDirOptions}

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