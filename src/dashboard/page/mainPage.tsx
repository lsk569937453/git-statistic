import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"

import { BaseInfoPage } from "./baseInfoPage";
import { useTranslation, Trans } from "react-i18next";
import { ActivityPage } from "./commitPage"
import { AuthorPage } from "./authorPage";
import { FileInfoPage } from "./fileInfoPage";
import { TagInfoPage } from "./tagInfoPage";
import { LineInfoPage } from "./lineInfoPage";
export default function MainPage() {
    const { t, i18n } = useTranslation();

    return (
        <Tabs defaultValue="general" className="w-full  h-[calc(100vh-60px)] p-10 flex flex-col	" >
            <TabsList className="grid w-3/4 grid-cols-6 flex-initial" >
                <TabsTrigger value="general">{t('mainPage.generalTabName')}</TabsTrigger>
                <TabsTrigger value="activity">{t('mainPage.activityTabName')}</TabsTrigger>
                <TabsTrigger value="authors">{t('mainPage.authorsTabName')}</TabsTrigger>
                <TabsTrigger value="files">{t('mainPage.fileTabName')}</TabsTrigger>
                <TabsTrigger value="lines">Lines</TabsTrigger>
                <TabsTrigger value="tag">{t('mainPage.tagTabName')}</TabsTrigger>

            </TabsList>
            <TabsContent value="general" className="w-full h-full"><BaseInfoPage /></TabsContent>
            <TabsContent value="activity" className="w-full h-full"><ActivityPage /></TabsContent>
            <TabsContent value="authors" className="w-full h-full"><AuthorPage /></TabsContent>
            <TabsContent value="files" className="w-full h-full"><FileInfoPage /></TabsContent>
            <TabsContent value="lines" className="w-full h-full"><LineInfoPage /></TabsContent>

            <TabsContent value="tag" className="w-full h-full"><TagInfoPage /></TabsContent>

        </Tabs>
    );
}