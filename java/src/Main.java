
void main() {
    //Tests der einzelnen Methoden
    System.out.println("---- TEST EAN-13 ----");

    String testEAN = "1347680943215";
    
    boolean eanErgebnis = RegexValidator.isValidEAN(testEAN);

    if(eanErgebnis){

        System.out.println ("Ja, " + testEAN + " ist ein gültiges EAN-Format.");
    }

    else{
        System.out.println ("Nein, " + testEAN + " ist kein gültiges EAN-Format.");
    }

    System.out.println("----- TEST QR CODE -----");

    String testQrCode = "ID:12345;PRICE:19.99;";

    String priceErgebnis = RegexValidator.extractPrice(testQrCode);

    if(priceErgebnis.equals("Kein Preis gefunden")){
        System.out.println("Achtung: Der Preis wurde im QR-Code nicht gefunden");
    }
    else{
        System.out.println("Der Preis ist: " + priceErgebnis + " Euro");
    }

    System.out.println("----- TEST URL -----");

    String testUrl = "http://dhsn.de";

    boolean urlErgebnis = RegexValidator.isURL(testUrl);

    if(urlErgebnis){

        System.out.println("Ja, " + testUrl + " ist eine gültige URL.");
    }
    else{
        System.out.println("Nein, " + testUrl + " ist keine gültige URL.");
    }

    System.out.println("----- TEST ISBN -----");

    String testISBN = "9781234567890";

    boolean isbnErgebnis = RegexValidator.isISBN(testISBN);

    if(isbnErgebnis){
        
        System.out.println("Ja, " + testISBN + " ist eine gültige ISBN");
    }
    else{
        System.out.println("Nein, " + testISBN + " ist keine gütlige ISBN");
    }
    
    System.out.println("----- TEST VCARD -----");

    String testVCARD = "BEGIN:VCARD\nFN:Paolo\nEND:VCARD";

    boolean vcardErgebnis = RegexValidator.isVCard(testVCARD);

    if(vcardErgebnis){

        System.out.println("Ja, ist eine gültige VCARD");
    }
    else{
        System.out.println("Nein, ist kein gültiges vCard-Format");
    }

    System.out.println("----- TEST WLAN -----");

    String testWLAN = "WIFI:S:MeinNetzwerk;P:Passwort123;;";

    String wlanErgebnis = RegexValidator.extractWifi(testWLAN);

    if(wlanErgebnis.equals("Kein Wlan gefunden")){

        System.out.println("Achtung: Der WLAN-Name wurde nicht gefunden.");
    }
    else{
        System.out.println("Der WLAN-Name ist: " + wlanErgebnis);
    }
}
