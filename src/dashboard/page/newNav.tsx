

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"

import { Base64TextPage } from "./base64TextPage";
import { Base64ImagePage } from './base64ImagePage';
import { useTranslation, Trans } from "react-i18next";
import { ActivityPage } from "./activityPage"

export default function Nav() {
  const { t, i18n } = useTranslation();

  return (<>
    <div className="sticky z-50 bg-gray-300 top-0 p-4">
      header contents
    </div>
    <div className="flex-grow">
      <main>
        <div>

        </div>
      </main>
    </div>
  </>
  );
}
