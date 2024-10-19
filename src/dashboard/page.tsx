
import { Button } from "@/components/ui/button"
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import {
    Tabs,
    TabsContent,
    TabsList,
    TabsTrigger,
} from "@/components/ui/tabs"
// import { CalendarDateRangePicker } from "@/dashboard/components/date-range-picker"
// import { MainNav } from "@/dashboard/components/main-nav"
// import { Overview } from "@/dashboard/components/overview"
// import { RecentSales } from "@/dashboard/components/recent-sales"
// import { Search } from "@/dashboard/components/search"
// import TeamSwitcher from "@/dashboard/components/team-switcher"
// import { UserNav } from "@/dashboard/components/user-nav"

import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { invoke } from "@tauri-apps/api/core";

import Sidebar from "./components/sidebar"
import { useState, useEffect } from "react"

import MainPage from "./page/mainPage"
import { useTranslation, Trans } from "react-i18next";
import Nav from "./page/newNav"

export default function DashboardPage() {
    const [selectedIndex, setSelectedIndex] = useState(0);
    const { t, i18n } = useTranslation();

    const [menulist, setMenulist] = useState<any>([]);

    useEffect(() => {
        const savedLanguage = localStorage.getItem('language');
        if (savedLanguage) {
            i18n.changeLanguage(savedLanguage);
        }
    }, []);


    return (
        <>
            <div className=" grid grid-cols-10 relative h-screen divide-x divide-foreground/30">
                <div className="col-span-10"><MainPage /></div>
            </div>
        </>
    )
}
