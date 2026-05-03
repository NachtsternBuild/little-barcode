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
}
