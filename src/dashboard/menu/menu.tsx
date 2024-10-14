"use client"

import { useCallback, useEffect, useState } from "react"
import logo from "@/assets/logo.png"
import { Globe, Mic, Sailboat } from "lucide-react"
// import { WindowTitlebar } from "tauri-controls"

import {
    Menubar,
    MenubarCheckboxItem,
    MenubarContent,
    MenubarItem,
    MenubarLabel,
    MenubarMenu,
    MenubarRadioGroup,
    MenubarRadioItem,
    MenubarSeparator,
    MenubarShortcut,
    MenubarSub,
    MenubarSubContent,
    MenubarSubTrigger,
    MenubarTrigger,
} from "@/components/ui/menubar"
import { open } from '@tauri-apps/plugin-dialog';

import { AboutDialog } from "./about-dialog"
import { PreferenceDialog } from "./preferenceDialog"
import { MenuModeToggle } from "./themModeMenu"
import { LanguageMenu } from "./languageMenu"
import { Dialog, DialogTrigger } from "../../components/ui/dialog"
import { Separator } from "@/components/ui/separator"
import { useTranslation, Trans } from "react-i18next";
import { CreateLinkDialog } from "./createLinkDialog";
import * as Progress from "@radix-ui/react-progress";
import { Button } from "flowbite-react";
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { invoke } from "@tauri-apps/api/core";

import { LoadingSpinner } from "../components/spinner";
import { useToast } from "@/components/ui/use-toast"
import { set } from "date-fns"

export function Menu() {

    const [showAboutDialog, setShowAboutDialog] = useState(false);
    const [showPreferenceDialog, setShowPreferenceDialog] = useState(false);
    const [showCreateLinkDialog, setShowCreateLinkDialog] = useState(false);
    const [showLoading, setShowLoading] = useState(false);
    const [responseCode, setResponseCode] = useState(-1);
    const [progressValue, setProgressValue] = useState(0);
    const [currentGitProcess, setCurrentGitProcess] = useState(0);
    const [totalGitProcess, setTotalGitProcess] = useState(0);
    const { t, i18n } = useTranslation();
    const { toast } = useToast()

    const buttonClick = async () => {
        const selected = await open({
            directory: true,
            multiple: false,

        });
        if (Array.isArray(selected)) {
            return;

        } else if (selected === null) {
            return;
        } else {
        };
        setShowLoading(true);
        const { response_code, response_msg } = JSON.parse(await invoke("init_git_async", { repoPath: selected }));
        console.log(response_code);
        console.log(response_msg);
        if (response_code == 0) {
            // window.location.reload();// 强制页面刷新
            setResponseCode(0);
        }
        else {
            setShowLoading(false);

            toast({
                variant: "destructive",
                title: t('toastMessage.errorMessageTile'),
                description: t('menu.gitPathInvalidMessageBody'),
            })
        }
        // console.log(selected);
        // setShowLoading(false);

    };
    const handleCancelButtonClick = async () => {
        const { response_code, response_msg } = JSON.parse(await invoke("cancel_init_task",));
        if (response_code == 0) {
            setShowLoading(false);
            window.location.reload();// 强制页面刷新
        }
    }
    const sleep = (time: any) => {
        return new Promise((resolve) => setTimeout(resolve, time));
    }
    useEffect(() => {
        let interval: any;
        console.log(responseCode);
        if (responseCode === 0) {
            interval = setInterval(async () => {
                try {
                    const { response_code, response_msg } = JSON.parse(await invoke("get_init_status"));
                    console.log(response_code);
                    console.log(response_msg);

                    if (response_code === 0) {
                        setCurrentGitProcess(response_msg[0]);
                        setTotalGitProcess(response_msg[1]);
                        if (response_msg[1] != 0) {
                            setProgressValue(Math.floor((response_msg[0] / response_msg[1]) * 100));
                        }
                        if (response_msg[0] === response_msg[1] && response_msg[1] !== 0) {
                            await sleep(500);
                            clearInterval(interval);
                            setShowLoading(false);
                            window.location.reload();// 强制页面刷新


                        }
                    }
                    // if (taskStatus === 'completed') {
                    //     clearInterval(interval); // Stop polling when task is complete
                    // } else if (taskStatus === 'failed') {
                    //     clearInterval(interval); // Stop polling if task failed
                    // }
                } catch (error) {
                    console.error('Error fetching task status:', error);
                    clearInterval(interval); // Stop polling if there's an error
                    setShowLoading(false);

                }
            }, 1000); // Poll every 5 seconds
        }

        return () => {
            if (interval) clearInterval(interval); // Cleanup on component unmount or task completion
        };
    }, [responseCode]);


    const saveHtml = () => {
        var pageHTML = document.documentElement.outerHTML;

        var tempEl = document.createElement('a');

        tempEl.href = 'data:attachment/text,' + encodeURI(pageHTML);
        tempEl.target = '_blank';
        tempEl.download = 'thispage.html';
        tempEl.click();
    }
    return (
        <div>
            <AlertDialog open={showLoading} onOpenChange={setShowLoading}>

                <AlertDialogContent className="w-30 bg-slate-200">
                    <AlertDialogTitle>{t("mainDialog.dialogTitle")}</AlertDialogTitle>
                    <div className="flex flex-col gap-4">
                        <p className="text-center">{t("mainDialog.dialogMainText")}</p>

                        <div className="flex flex-row gap-x-4">
                            <Progress.Root
                                className="relative h-[25px] w-[300px] overflow-hidden rounded-full bg-slate-300	"
                                style={{

                                    transform: "translateZ(0)",
                                }}
                                value={progressValue}
                            >
                                <Progress.Indicator
                                    className="ease-[cubic-bezier(0.65, 0, 0.35, 1)] w-full h-full bg-green-500 transition-transform duration-\[660ms\]"
                                    style={{ transform: `translateX(-${100 - progressValue}%)` }}
                                />
                            </Progress.Root>
                            {/* <p>{progressValue}%</p><p className="font-bold">[{currentGitProcess}/{totalGitProcess}]</p> */}
                        </div>
                        <div className="grid grid-cols-2 gap-4">
                            {i18n.language === 'zh' && <>
                                <p className="text-right">状态:</p>
                                <p>{progressValue}% 完成</p>
                            </>}
                            {i18n.language === 'en' && <>
                                <p className="text-right">Status:</p>
                                <p>{progressValue}% completed</p>
                            </>}

                            <p className="text-right">{t("mainDialog.taskStatus")}:</p>
                            <p>{currentGitProcess}</p>

                            <p className="text-right">{t("mainDialog.taskStatus")}:</p>
                            <p>{totalGitProcess}</p>
                        </div>

                    </div>
                    <Button color="failure" onClick={() => handleCancelButtonClick()}>
                        取消任务
                    </Button>
                </AlertDialogContent>
            </AlertDialog>
            <Menubar className="rounded-none border-b border-none pl-2 lg:pl-3">
                <MenubarMenu>

                </MenubarMenu>
                <Dialog open={showAboutDialog} onOpenChange={setShowAboutDialog}>
                    <AboutDialog />
                </Dialog>

                <Dialog open={showPreferenceDialog} onOpenChange={setShowPreferenceDialog}>
                    <PreferenceDialog />
                </Dialog>
                <Dialog open={showCreateLinkDialog} onOpenChange={setShowCreateLinkDialog} >
                    <CreateLinkDialog />
                </Dialog>
                <MenubarMenu>
                    <MenubarTrigger className="font-bold">{t('toolBar.app.name')}</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={() => setShowAboutDialog(true)}>{t('toolBar.app.first_item')}</MenubarItem>
                        {/* <MenubarSeparator /> */}
                        {/* <MenubarItem onClick={() => setShowPreferenceDialog(true)}>
                            {t('toolBar.app.second_item')}
                        </MenubarItem> */}
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu>
                    <MenubarTrigger className="font-bold">{t('toolBar.configuration.name')}</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={() => buttonClick()}>{t('toolBar.configuration.first_item')}</MenubarItem>
                        {/* <MenubarSeparator />
            <MenubarItem onClick={() => saveHtml()}>
              保存html
            </MenubarItem> */}
                    </MenubarContent>
                </MenubarMenu>
                {/* <MenuModeToggle /> */}
                <LanguageMenu />
            </Menubar>
        </div >
    )
}
