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
import { set } from "date-fns";
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

export function TagInfoPage() {
    const { t, i18n } = useTranslation();

    const [currentInput, setCurrentInput] = useState();
    const { toast } = useToast()
    const [tagBaseInfo, setTagBaseInfo] = useState<any>();
    const [tableData, setTableData] = useState<any>([]);

    useEffect(() => {
        loadData();
    }, [])
    const loadData = async () => {

        const { response_code, response_msg } = JSON.parse(await invoke("get_tag_info"));
        console.log(response_code);
        console.log(response_msg);

        if (response_code === 0) {
            const { tag_statistic_base_info, tag_statistic_ext_info

            } = response_msg;
            let fileBaseInfo = JSON.parse(tag_statistic_base_info);
            console.log(fileBaseInfo);
            setTagBaseInfo(fileBaseInfo);

            let fileMainInfo = JSON.parse(tag_statistic_ext_info).list;
            console.log(fileMainInfo);
            setTableData(fileMainInfo);
        }
    }
    const renderTable = () => {
        return tableData.map((item: any, index: any) => {

            let authorsString = item.authors.map((itemx: any) => `${itemx[0]}(${itemx[1]})`).join(',');
            return <TableRow key={index} className="w-full">
                <TableCell className="text-center border border-black ">{item.tag_name}</TableCell>
                <TableCell className="text-center border border-black ">{item.date} </TableCell>
                <TableCell className="text-center border border-black ">{item.commit_count} </TableCell>
                <TableCell className="text-center border border-black ">
                    {authorsString}
                </TableCell>
            </TableRow>
        });
    }
    return (
        <div className="flex flex-col  	overflow-y-auto h-full	overflow-x-hidden">
            <Card className="pt-4 w-full">
                <CardContent className="flex flex-col gap-5 text-right">
                    <div className="flex flex-row gap-10">
                        <p className="basis-3/12 text-lg font-bold">Total tags:</p>
                        <p className="text-lg">{tagBaseInfo?.total_tags}</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-3/12 text-lg font-bold">Average commits per tag:</p>
                        <p className="text-lg">{tagBaseInfo?.average_commit_per_tag}</p>
                    </div>

                    <Separator />
                    <Table className="w-full table-fixed overflow-x-hidden">
                        <TableHeader>
                            <TableRow>
                                <TableHead className="text-center text-blue-500 font-bold border-y border-l border-black w-1/12">Name</TableHead>
                                <TableHead className="text-center text-blue-500 font-bold border-y	border-black w-1/12">Date</TableHead>

                                <TableHead className="text-center text-blue-500 font-black border-y border-black w-1/12">Commits</TableHead>
                                <TableHead className="text-center text-blue-500 font-black border-y border-r	border-black w-3/4">Authors</TableHead>

                            </TableRow>

                        </TableHeader>
                        <TableBody className="w-full">

                            {renderTable()}
                        </TableBody>
                    </Table>
                </CardContent>

            </Card>
        </div>

    );
}