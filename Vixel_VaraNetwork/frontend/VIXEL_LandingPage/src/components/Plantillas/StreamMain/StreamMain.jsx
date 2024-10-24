import "./StreamMain.css";
import ImgStream1 from "../../../img/portada-stream-1.png";
import ImgStream2 from "../../../img/portada-stream-2.png";
import ImgStream3 from "../../../img/portada-stream-3.png";
import ImgStream4 from "../../../img/portada-stream-4.png";
import FrontPageStream from "@/components/Atoms/FrontPageStream/FrontPageStream";

export default function StreamMain (){

    const objStream1 = {
        srcPortada: ImgStream1,
        views: 500,
        title: "Saco mi modo crack en vivo - whaleXpace",
        nameStreamer: "Fernanfloo"
    }

    const objStream2 = {
        srcPortada: ImgStream2,
        views: 666,
        title: "Nunca jueges Coin Drifters a las 3:33 am",
        nameStreamer: "Sobrenatural"
    }

    const objStream3 = {
        srcPortada: ImgStream3,
        views: 207,
        title: "planeta wigeta ep 30",
        nameStreamer: "Wigeta 117"
    }

    const objStream4 = {
        srcPortada: ImgStream4,
        views: 1076,
        title: "PVP insano wasa",
        nameStreamer: "User pvp"
    }

    return(
        <main className="body-streams-section">
            <div className="search-section"></div>
            <div className="content-streams">
                <FrontPageStream objStream={objStream1}/>
                <FrontPageStream objStream={objStream2}/>
                <FrontPageStream objStream={objStream3}/>
                <FrontPageStream objStream={objStream4}/>
            </div>
            <div className="content-streams">
                <FrontPageStream objStream={objStream1}/>
                <FrontPageStream objStream={objStream2}/>
                <FrontPageStream objStream={objStream3}/>
                <FrontPageStream objStream={objStream4}/>
            </div>
            <div className="content-streams">
                <FrontPageStream objStream={objStream1}/>
                <FrontPageStream objStream={objStream2}/>
                <FrontPageStream objStream={objStream3}/>
                <FrontPageStream objStream={objStream4}/>
            </div>
        </main>
    )
}