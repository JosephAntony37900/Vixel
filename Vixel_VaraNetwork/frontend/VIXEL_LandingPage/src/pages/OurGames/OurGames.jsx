import React from "react";
import "./OurGames.css";
import HeaderPage from "@/components/Organismos/HeaderPage/HeaderPage";
import FooterPage from "@/components/Organismos/FooterPage/FooterPage";
import GameInfo from "@/components/Plantillas/GameInfo/GameInfo";
import RgbLine from "@/components/Atoms/RgbLine/RgbLine";

export default function OurGames (){
    return(
        <main>
            <HeaderPage/>
            <diV>
                <GameInfo/>
                <RgbLine/>
            </diV>
            <FooterPage/>
        </main>
    );
}