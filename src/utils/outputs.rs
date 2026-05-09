const INCORRECT_CALL_PROMPT: &str = "
Fehlerhafter Programmaufruf!
Verwendung: hundeschule.exe /n | /imp | /exp
";

const TEAM: &str = "
Denys Stamat - Kate Lagutin - Malcolm Apaloo
";

const HELP: &str = "
Verwendung:

    hundeschule.exe [ARGUMENT]

Mögliche Argumente:

    /n          Gibt die Namen aller Beteiligten aus
    /imp        Importiert PDF-Formulare in Datenbank
    /exp        Für jeden Kurs exportiert eine CSV-Liste mit Teilnehmer 

Hinweise:

    - Es werden nur vollständig ausgefüllte Formulare importiert. Nicht vollständige Formulare werden ignoriert.
    - Kurse ohne Teilnehmer werden bei dem Export ignoriert. Für diese wird keine CSV-Datei erstellt.
    - Ein Hundehalter, ein Hund und ein Kurs wird nur EINMALIG importiert. Bereits importierte Entitäten werden ignoriert.


    Dieses Program ist nur auf Windows 11 lauffähig.
";

pub fn write_incorrect_call() -> anyhow::Result<()> {
    println!("{}", &INCORRECT_CALL_PROMPT);
    Ok(())
}

pub fn write_team() -> anyhow::Result<()> {
    println!("{}", &TEAM);
    Ok(())
}

pub fn write_help() -> anyhow::Result<()> {
    println!("{}", &HELP);
    Ok(())
}
