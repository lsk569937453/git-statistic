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
import { open } from '@tauri-apps/api/dialog';

import { AboutDialog } from "./about-dialog"
import { PreferenceDialog } from "./preferenceDialog"
import { MenuModeToggle } from "./themModeMenu"
import { LanguageMenu } from "./languageMenu"
import { Dialog, DialogTrigger } from "../../components/ui/dialog"
import { Separator } from "@/components/ui/separator"
import { useTranslation, Trans } from "react-i18next";
import { CreateLinkDialog } from "./createLinkDialog"
export function Menu() {

  const [showAboutDialog, setShowAboutDialog] = useState(false);
  const [showPreferenceDialog, setShowPreferenceDialog] = useState(false);
  const [showCreateLinkDialog, setShowCreateLinkDialog] = useState(false);
  const { t, i18n } = useTranslation();
  const buttonClick = async () => {
    const selected = await open({
      directory: true,
      multiple: false,

    });
    if (Array.isArray(selected)) {
    } else if (selected === null) {
    } else {
    };
    console.log(selected);
  };
  const saveHtml = () => {
    var pageHTML = document.documentElement.outerHTML;

    var tempEl = document.createElement('a');

    tempEl.href = 'data:attachment/text,' + encodeURI(pageHTML);
    tempEl.target = '_blank';
    tempEl.download = 'thispage.html';
    tempEl.click();
  }
  return (
    <div
    >
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
            <MenubarSeparator />
            <MenubarItem onClick={() => setShowPreferenceDialog(true)}>
              {t('toolBar.app.second_item')}
            </MenubarItem>
          </MenubarContent>
        </MenubarMenu>
        <MenubarMenu>
          <MenubarTrigger className="font-bold">配置</MenubarTrigger>
          <MenubarContent>
            <MenubarItem onClick={() => buttonClick()}>选择git仓库地址</MenubarItem>
            {/* <MenubarSeparator />
            <MenubarItem onClick={() => saveHtml()}>
              保存html
            </MenubarItem> */}
          </MenubarContent>
        </MenubarMenu>
        {/* <MenuModeToggle /> */}
        <LanguageMenu />
      </Menubar>
    </div>
  )
}
