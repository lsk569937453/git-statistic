import { Textarea } from "@/components/ui/textarea"
import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button"
import { useToast } from "@/components/ui/use-toast"
import { useTranslation, Trans } from "react-i18next";
import { Separator } from "@/components/ui/separator"

import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"

import ReactECharts from 'echarts-for-react';
import ReactEChartsCore from 'echarts-for-react/lib/core';
// Import the echarts core module, which provides the necessary interfaces for using echarts.
import * as echarts from 'echarts/core';
import {
    // LineChart,
    BarChart,
    // PieChart,
    // ScatterChart,
    // RadarChart,
    // MapChart,
    // TreeChart,
    // TreemapChart,
    // GraphChart,
    // GaugeChart,
    // FunnelChart,
    // ParallelChart,
    // SankeyChart,
    // BoxplotChart,
    // CandlestickChart,
    // EffectScatterChart,
    // LinesChart,
    // HeatmapChart,
    // PictorialBarChart,
    // ThemeRiverChart,
    // SunburstChart,
    // CustomChart,
} from 'echarts/charts';
// import components, all suffixed with Component
import {
    // GridSimpleComponent,
    GridComponent,
    // PolarComponent,
    // RadarComponent,
    // GeoComponent,
    // SingleAxisComponent,
    // ParallelComponent,
    // CalendarComponent,
    // GraphicComponent,
    // ToolboxComponent,
    TooltipComponent,
    // AxisPointerComponent,
    // BrushComponent,
    TitleComponent,
    // TimelineComponent,
    // MarkPointComponent,
    // MarkLineComponent,
    // MarkAreaComponent,
    // LegendComponent,
    // LegendScrollComponent,
    // LegendPlainComponent,
    // DataZoomComponent,
    // DataZoomInsideComponent,
    // DataZoomSliderComponent,
    // VisualMapComponent,
    // VisualMapContinuousComponent,
    // VisualMapPiecewiseComponent,
    // AriaComponent,
    // TransformComponent,
    DatasetComponent,
} from 'echarts/components';
import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
// Import renderer, note that introducing the CanvasRenderer or SVGRenderer is a required step
import {
    CanvasRenderer,
    // SVGRenderer,
} from 'echarts/renderers';
import { Divide } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area"

// Register the required components
echarts.use(
    [TitleComponent, TooltipComponent, GridComponent, BarChart, CanvasRenderer]
);
export function AuthorPage() {
    const { t, i18n } = useTranslation();

    const [currentInput, setCurrentInput] = useState();
    const { toast } = useToast()
    const [totalAuthors, setTotalAuthors] = useState([]);
    const [authorOfMonth, setAuthorOfMonth] = useState([]);
    const [authorOfYear, setAuthorOfYear] = useState([]);

    useEffect(() => {
        loadData();
    }, []);
    const loadData = async () => {
        const { response_code, response_msg } = JSON.parse(await invoke("get_authors_info"));
        if (response_code === 0) {
            const { author_of_month_statistic_info, author_of_year_statistic_info, total_authors_statistic_info
            } = response_msg;
            console.log(author_of_month_statistic_info);
            console.log(author_of_year_statistic_info);
            console.log(total_authors_statistic_info);
            let totalAuthors = JSON.parse(total_authors_statistic_info);
            setTotalAuthors(totalAuthors);
            let authorOfMonth = JSON.parse(author_of_month_statistic_info).data;
            setAuthorOfMonth(authorOfMonth);
            let authorOfYear = JSON.parse(author_of_year_statistic_info).data;
            setAuthorOfYear(authorOfYear);
            // let recentWeeksCommit = JSON.parse(recent_weeks_commit).commits_map;
            // setRecentWeeksCommit(recentWeeksCommit);
            // let hoursOfDayCommit = JSON.parse(hours_of_day_commit).commits_map;
            // setHoursOfDayCommit(hoursOfDayCommit);

            // let monthOfYearCommit = JSON.parse(month_of_year_commit).commits_map;
            // setMonthOfYearCommit(monthOfYearCommit);

            // let dayOfWeekCommit = JSON.parse(day_of_week).commits_map;
            // setDayOfWeekCommit(dayOfWeekCommit);

            // let yearAndMonthCommit = JSON.parse(year_and_month_commit).commits_map;
            // setYearAndMonthCommit(yearAndMonthCommit);

            // let yearCommit = JSON.parse(year_commit).commits_map;
            // setYearCommit(yearCommit);
        }
    }
    const handleValueChange = (e: any) => {
        setCurrentInput(e.target.value);
    }
    const recent32WeekOp = () => {

        return {
            title: {
                text: '最近32周活动',
            },
            xAxis: {
                type: 'category',
                data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
            },
            yAxis: {
                type: 'value'
            },
            series: [
                {
                    data: [120, 200, 150, 80, 70, 110, 130],
                    type: 'bar'
                }
            ]
        };
    };
    const renderTable = () => {
        return totalAuthors.map((item: any, index) => {
            return <TableRow>
                <TableCell className="text-center border border-black">{item.author_name}</TableCell>
                <TableCell className="text-center border border-black">{item.total_commit} </TableCell>
                <TableCell className="text-center border border-black">{item.total_added} </TableCell>
                <TableCell className="text-center border border-black">{item.total_deleted} </TableCell>
                <TableCell className="text-center border border-black">{item.first_commit} </TableCell>
                <TableCell className="text-center border border-black">{item.last_commit} </TableCell>
                <TableCell className="text-center border border-black">{item.age} days </TableCell>
                <TableCell className="text-center border border-black">{item.active_days} </TableCell>

            </TableRow>
        });
    }
    const renderTableOfMonth = () => {

        return authorOfMonth.map((item: any, index) => {
            let result = (item.count_of_commit_of_author / item.total_commit_count) * 100;
            let formattedResult = result.toFixed(2) + "%"
            return <TableRow>
                <TableCell className="text-center border border-black">{item.date}</TableCell>
                <TableCell className="text-center border border-black">{item.author_name} </TableCell>
                <TableCell className="text-center border border-black">{item.count_of_commit_of_author}({formattedResult} of {item.total_commit_count}) </TableCell>
                <TableCell className="text-center border border-black">{item.next_top_five.join(", ")} </TableCell>
                <TableCell className="text-center border border-black">{item.count_of_author} </TableCell>

            </TableRow>
        });
    }
    const renderTableOfYear = () => {

        return authorOfYear.map((item: any, index) => {
            let result = (item.count_of_commit_of_author / item.total_commit_count) * 100;
            let formattedResult = result.toFixed(2) + "%"
            return <TableRow>
                <TableCell className="text-center border border-black">{item.date}</TableCell>
                <TableCell className="text-center border border-black">{item.author_name} </TableCell>
                <TableCell className="text-center border border-black">{item.count_of_commit_of_author}({formattedResult} of {item.total_commit_count}) </TableCell>
                <TableCell className="text-center border border-black">{item.next_top_five.join(", ")} </TableCell>
                <TableCell className="text-center border border-black">{item.count_of_author} </TableCell>

            </TableRow>
        });
    }
    return (

        <ScrollArea className="h-full">
            <Card className="px-10 h-full">

                <div className="p-4 flex flex-col gap-5 text-right ">
                    <div className="flex flex-col gap-y-4">
                        <p className="text-2xl font-bold text-left">List Of Authors</p>
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <TableHead className="text-center text-blue-500 font-bold border-y border-l border-black">Authors</TableHead>
                                    <TableHead className="text-center text-blue-500 font-bold border-y	border-black">Total Commit</TableHead>

                                    <TableHead className="text-center text-blue-500 font-black border-y	border-black ">Total Added</TableHead>
                                    <TableHead className="text-center text-blue-500 font-bold border-y border-black">Total Deleted</TableHead>
                                    <TableHead className="text-center text-blue-500 font-bold border-y border-black" >First Commit</TableHead>
                                    <TableHead className="text-center text-blue-500 font-bold border-y border-black">Last Commit</TableHead>
                                    <TableHead className="text-center text-black-500 font-bold border-y border-black">Age</TableHead>
                                    <TableHead className="text-center text-blue-500 font-bold border-y  border-r border-black">Active Days</TableHead>
                                </TableRow>

                            </TableHeader>
                            <TableBody>

                                {renderTable()}
                            </TableBody>
                        </Table>
                    </div>
                    <Separator />
                    <div className="flex flex-col gap-y-4">
                        <p className="text-2xl font-bold text-left">Author Of Month</p>
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <TableHead className="text-center text-blue-500 font-bold border-y border-l border-black">Month</TableHead>
                                    <TableHead className="text-center text-blue-500 font-bold border-y	border-black">Author</TableHead>

                                    <TableHead className="text-center text-blue-500 font-black border-y	border-black ">Commits</TableHead>
                                    <TableHead className="text-center text-black-500 font-black border-y	border-black ">Next Top5</TableHead>
                                    <TableHead className="text-center text-blue-500 font-black border-y	border-black ">Number Of Authors</TableHead>



                                </TableRow>

                            </TableHeader>
                            <TableBody>

                                {renderTableOfMonth()}
                            </TableBody>
                        </Table>
                    </div>
                    <Separator />

                    <div className="flex flex-col gap-y-4">
                        <p className="text-2xl font-bold text-left">Author Of Year</p>
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <TableHead className="text-center text-blue-500 font-bold border-y border-l border-black">Month</TableHead>
                                    <TableHead className="text-center text-blue-500 font-bold border-y	border-black">Author</TableHead>

                                    <TableHead className="text-center text-blue-500 font-black border-y	border-black ">Commits</TableHead>
                                    <TableHead className="text-center text-black-500 font-black border-y	border-black ">Next Top5</TableHead>
                                    <TableHead className="text-center text-blue-500 font-black border-y	border-black ">Number Of Authors</TableHead>



                                </TableRow>

                            </TableHeader>
                            <TableBody>

                                {renderTableOfYear()}
                            </TableBody>
                        </Table>
                    </div>

                </div>
            </Card>

        </ScrollArea>

    );
}