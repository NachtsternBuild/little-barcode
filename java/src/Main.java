
void main() {
    System.out.println("---- TEST EAN-13 ----");

    String testEAN = "1347680943215";
    
    boolean eanErgebnis = RegexValidator.isValidEAN(testEAN);

    if(eanErgebnis){

        System.out.println ("Ja," + testEAN + " ist ein gültiges EAN-Format.");
    }

    else{
        System.out.println ("Nein," + testEAN + " ist kein gültiges EAN-Format.");
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

}
