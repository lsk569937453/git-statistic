"use client"

import * as React from "react"
import { LaptopIcon, MoonIcon, SunIcon } from "@radix-ui/react-icons"
import { useTheme } from "next-themes"

import { Button } from "@/components/ui/button"
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import {
    MenubarContent,
    MenubarMenu,
    MenubarRadioGroup,
    MenubarRadioItem,
    MenubarTrigger,
} from "@/components/ui/menubar"
import { Icons } from "@/dashboard/menu/icons"
import { useTranslation, Trans } from "react-i18next";
import { invoke } from "@tauri-apps/api/core"

export function LanguageMenu() {
    const { t, i18n } = useTranslation();
    const invokeSetLanguage = async (lang: string) => {
        const { response_code, response_msg } = JSON.parse(await invoke("set_language", { language: lang }));

    }
    return (
        <MenubarMenu>
            <MenubarTrigger className="font-bold">{t('toolBar.language.name')}</MenubarTrigger>
            <MenubarContent forceMount>
                <MenubarRadioGroup value={i18n.language}>
                    <MenubarRadioItem value="en" onClick={() => {
                        i18n.changeLanguage("en");
                        localStorage.setItem('language', 'en');
                        invokeSetLanguage("en");


                    }}>
                        <span>{t('toolBar.language.english')}</span>
                    </MenubarRadioItem>
                    <MenubarRadioItem value="zh" onClick={() => {
                        i18n.changeLanguage("zh");
                        localStorage.setItem('language', 'zh');
                        invokeSetLanguage("zh");

                    }}>
                        <span>{t('toolBar.language.chinese')}</span>
                    </MenubarRadioItem>
                </MenubarRadioGroup>
            </MenubarContent>
        </MenubarMenu>
    )
}
