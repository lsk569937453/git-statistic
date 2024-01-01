import { Textarea } from "@/components/ui/textarea"
import { useState } from "react"
import { invoke } from "@tauri-apps/api/tauri";
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


export function Base64TextPage() {
    const { t, i18n } = useTranslation();

    const [currentInput, setCurrentInput] = useState();
    const { toast } = useToast()

    const base64Encode = async () => {
        i18n.changeLanguage("zh");
        if (currentInput === undefined || currentInput === "") {
            toast({
                variant: "destructive",
                title: t('toastMessage.errorMessageTile'),
                description: t('base64TextPage.sourceTextNotEmptyMessageBody'),
            })
            return;
        }
        const { response_code, response_msg } = JSON.parse(await invoke("base64_encode", { sourceString: currentInput }));
        console.log(response_code);
        console.log(response_msg);

        if (response_code === 0) {
            setCurrentInput(response_msg);
        }
    }
    const base64Decode = async () => {
        i18n.changeLanguage("en");

        if (currentInput === undefined || currentInput === "") {
            toast({
                variant: "destructive",
                title: t('toastMessage.errorMessageTile'),
                description: t('base64TextPage.sourceTextNotEmptyMessageBody'),
            })
            return;
        }
        const { response_code, response_msg } = JSON.parse(await invoke("base64_decode", { sourceString: currentInput }));
        console.log(response_code);
        console.log(response_msg);

        if (response_code === 0) {
            setCurrentInput(response_msg);
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
                        <p className="basis-2/12 text-lg font-bold">项目名称:</p>
                        <p className="text-lg">xxxxx</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-2/12 text-lg font-bold">报告生成时间:</p>
                        <p className="text-lg">xxxxx</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-2/12 text-lg font-bold">项目周期:</p>
                        <p className="text-lg">xxxxx</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-2/12 text-lg font-bold">项目文件数量:</p>
                        <p className="text-lg">xxxxx</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-2/12 text-lg font-bold">总代码行数:</p>
                        <p className="text-lg">xxxxx</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-2/12 text-lg font-bold">总Commit数量:</p>
                        <p className="text-lg">xxxxx</p>
                    </div>
                    <div className="flex flex-row gap-10 text-right">
                        <p className="basis-2/12 text-lg font-bold">项目参与人:</p>
                        <p className="text-lg">xxxxx</p>
                    </div>
                </CardContent>

            </Card>
        </div>

    );
}