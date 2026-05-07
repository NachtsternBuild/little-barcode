import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class RegexValidator {
    
    // Überprüfe, ob ein EAN-13-Barcode gültig ist
    // d. h., ob er genau 13 Ziffern enthält.
    public static boolean isValidEAN(String ean){
        
        //Erstellung Regex:
        // ^ = Anfang String, \\d{13} = genau 13 Ziffer, $=ende String 
        String regex = "^\\d{13}$";

        Pattern pattern = Pattern.compile(regex);

        //Vergleiche Regex mit dem Parameter.
        Matcher matcher = pattern.matcher(ean);

        //Er gibt "true" zurück, wenn er übereinstimmt, sonst "false"
        return matcher.matches();
    }

    //Methode zum Extrahieren des Preises aus einem QR-Code
    public static String extractPrice (String qrData){

        //Erstellung Regex:
        // Price= sucht nach Text PREISE, ()=gefangene Gruppe, 
        // \\d+= 1 oder mehrere Ziffern, \\[.,] = Dezimalpunkt, \\d+ = Ziffern nach Komma 
        String regex = "PRICE:(\\d+[.,]\\d+)";

        Pattern pattern = Pattern.compile(regex);

        //Suche nach dem Regex im Text des QR-Codes
        Matcher matcher = pattern.matcher(qrData);

        //find() sucht nach einem Textabschnitt, der dem Regex entspricht.
        if(matcher.find()){

            //group(1) gibt den Inhalt der ersten Gruppe zurück,
            //d.h. den Preis ohne Wort "Preis"
            return matcher.group(1);
        }

         return "Kein Preis gefunden";
    }

    public static boolean isURL(String urlText){
        // Erstellung regex:
        // beginnt mit: http o https, dann :// anderen Inhalt
        String regex = "^https?://.*";

        return urlText.matches(regex);
    }

    public static boolean isISBN(String isbnCode){
        //Erstellung regex:
        // beginnt mit 978 o 979 und dann noch 10 Ziffern (13 insgesamt)
        String regex =  "^(978|979)\\d{10}$";

        return isbnCode.matches(regex);
    }

    public static boolean isVCard(String contactData){
        //Erstellung regex:
        //(?s): mit diesem liest Regex mehr textreihe, ^ = Anfang String, $=ende String  
        String regex = "(?s)^BEGIN:VCARD.*END:VCARD$";

        return contactData.matches(regex);
    }

    public static String extractWifi (String wifiRawData){
        //Erstellung regex:
        //Sucht nach pattern Wi-Fi und nimmt was zwischen ' S:' und ' ; ' ist
        String regex = "WIFI:S:(.*?);";

        Pattern pattern = Pattern.compile(regex);

        //Suche nach dem Regex im Text des QR-Codes
        Matcher matcher = pattern.matcher(wifiRawData);
        if (matcher.find()) {
        return matcher.group(1); 
    }
    return "Kein WLan gefunden";
    }

    public static String identifyContent(String rawData){

        //kontrolliert ob es um EAN-13 geht:
        if(isValidEAN(rawData)){
            return ("Datentyp: Handelprodukt (EAN-13)");
        }
        //kontrolliert ob es um URL geht:
        if(isURL(rawData)){
            return ("Datentyp: Web-Link (URL)");
        }
        //kontrolliert ob es um eine ISBN geht:
        if(isISBN(rawData)){
            return ("Datentyp: Buch (ISBN)");
        }

        //kontrolliert ob es um eine vCard geht:
        if(isVCard(rawData)){
            return ("DATENTYP: Visitenkarte (vCard)");
        }

        //kontrolliert ob es um eine QR-CODE geht:
        String price = extractPrice(rawData);
        if(!price.equals("Kein Preis gefunden")){
            return ("DATENTYP: QR-Code mit Preisinfo (Preis: " + price + " Euro");
        }

        //kontrolliert ob es um eine Wi-Fi geht:
        String wlanName = extractWifi(rawData);
        if(!wlanName.equals("Kein WLan gefunden")){
            return ("DATENTYP: WLAN (Netzwerk: " + wlanName + ")");
        }

        return "ungültigen Datentyp: Unbekannter Text / Allgemeiner Inhalt";
    }
}
