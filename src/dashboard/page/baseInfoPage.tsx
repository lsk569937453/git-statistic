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


export function BaseInfoPage() {
    const { t, i18n } = useTranslation();

    const [currentInput, setCurrentInput] = useState();
    const { toast } = useToast()
    const [baseInfo, setBaseInfo] = useState<any>();
    useEffect(() => {
        loadData();
    }, [])
    const loadData = async () => {

        const { response_code, response_msg } = JSON.parse(await invoke("get_base_info"));
        console.log(response_code);
        console.log(response_msg);

        if (response_code === 0) {
            setBaseInfo(response_msg);
        }
    }

    const handleValueChange = (e: any) => {
        setCurrentInput(e.target.value);
    }
    return (
        <div className="flex flex-col">
            <Card className="pt-4">
                <CardContent className="flex flex-col gap-5 text-right">
                    <div className="flex flex-row gap-10">
                        <p className="basis-3/12 text-lg font-bold">{t('generalPage.projectName')}:</p>
                        <p className="text-lg">{baseInfo?.project_name}</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-3/12 text-lg font-bold">{t('generalPage.reportGeneratorTime')}:</p>
                        <p className="text-lg">{baseInfo?.generate_time}</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-3/12 text-lg font-bold">{t('generalPage.gitStatisticDateRange')}:</p>
                        <p className="text-lg"><span className="mr-2">{baseInfo?.first_commit_time}</span> 至
                            <span className="ml-2">{baseInfo?.last_commit_time}</span>
                        </p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-3/12 text-lg font-bold">{t('generalPage.projectRange')}:</p>
                        <p className="text-lg"><span className="mr-2"> {baseInfo?.age}</span>天</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-3/12 text-lg font-bold">{t('generalPage.fileCount')}:</p>
                        <p className="text-lg">{baseInfo?.total_files}</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-3/12 text-lg font-bold">{t('generalPage.totalLines')}:</p>
                        <p className="flex items-center"
                        >
                            <span className="text-lg mr-2">{baseInfo?.total_lines}</span>
                            {i18n.language === 'zh' && <>
                                (添加
                                <span className="text-green-600 inline-block mx-2">{baseInfo?.total_added}</span> 行,删除
                                <span className="text-red-600 inline-block mx-2">{baseInfo?.total_deleted}</span>
                                行)
                            </>}
                            {i18n.language === 'en' && <>
                                (
                                <span className="text-green-600 inline-block mx-2">{baseInfo?.total_added}</span> added,
                                <span className="text-red-600 inline-block mx-2">{baseInfo?.total_deleted}</span> removed
                                )
                            </>}
                        </p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-3/12 text-lg font-bold">{t('generalPage.totalCommits')}:</p>
                        <p className="text-lg">{baseInfo?.total_commits}</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-3/12 text-lg font-bold">{t('generalPage.totalAuthors')}:</p>
                        <p className="text-lg">{baseInfo?.authors}</p>
                    </div>
                </CardContent>

            </Card>
        </div >

    );
}