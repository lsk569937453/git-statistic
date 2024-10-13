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

export function FileInfoPage() {
    const { t, i18n } = useTranslation();

    const [currentInput, setCurrentInput] = useState();
    const { toast } = useToast()
    const [fileBaseInfo, setFileBaseInfo] = useState<any>();
    const [tableData, setTableData] = useState<any>([]);

    useEffect(() => {
        loadData();
    }, [])
    const loadData = async () => {

        const { response_code, response_msg } = JSON.parse(await invoke("get_files_info"));
        console.log(response_code);
        console.log(response_msg);

        if (response_code === 0) {
            const { files_statistic_ext_info, files_statistic_base_info
            } = response_msg;
            let fileBaseInfo = JSON.parse(files_statistic_base_info);
            console.log(fileBaseInfo);
            setFileBaseInfo(fileBaseInfo);

            let fileMainInfo = JSON.parse(files_statistic_ext_info).list;
            console.log(fileBaseInfo);
            setTableData(fileMainInfo);
        }
    }
    const renderTable = () => {
        return tableData.map((item: any, index: any) => {
            return <TableRow>
                <TableCell className="text-center border border-black">{item.extention_name}</TableCell>
                <TableCell className="text-center border border-black">{item.files_count} </TableCell>
                <TableCell className="text-center border border-black">{item.lines_count} </TableCell>


            </TableRow>
        });
    }
    return (
        <ScrollArea className="h-full">
            <div className="flex flex-col">
                <Card className="pt-4">
                    <CardContent className="flex flex-col gap-5 text-right">
                        <div className="flex flex-row gap-10">
                            <p className="basis-2/12 text-lg font-bold">{t("filePage.totalFiles")}:</p>
                            <p className="text-lg">{fileBaseInfo?.total_files_count}</p>
                        </div>
                        <div className="flex flex-row gap-10 text-right">
                            <p className="basis-2/12 text-lg font-bold">{t("filePage.totalLines")}:</p>
                            <p className="text-lg">{fileBaseInfo?.total_lines_count}</p>
                        </div>
                        <div className="flex flex-row gap-10 text-right">
                            <p className="basis-2/12 text-lg font-bold">{t("filePage.averageFileSize")}:</p>
                            <p className="text-lg">{fileBaseInfo?.average_file_size} bytes</p>
                        </div>
                        <Separator />
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <TableHead className="text-center text-blue-500 font-bold border-y border-l border-black">{t("filePage.extension")}</TableHead>
                                    <TableHead className="text-center text-blue-500 font-bold border-y	border-black">{t("filePage.filesCount")}</TableHead>

                                    <TableHead className="text-center text-blue-500 font-black border-y border-r	border-black ">{t("filePage.linesCount")}</TableHead>

                                </TableRow>

                            </TableHeader>
                            <TableBody>

                                {renderTable()}
                            </TableBody>
                        </Table>
                    </CardContent>

                </Card>
            </div>
        </ScrollArea>

    );
}