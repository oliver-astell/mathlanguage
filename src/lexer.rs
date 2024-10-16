use project::Project;


const VARIABLES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SING_VARS: &str = "αβγδεϝζηθϑκϰλμξϖρϱσςϕφψωΓΔϜΘΛΞΟΠΡΣΦΨΩπτ";
// reserved     ϵ
// identicals   ηινουχ ΑΒΕΖΗΙΚΜΝΤΥΧ

const NUMERALS: &str = "0123456789";

fn lex(project: Project) {
    let source = project.read();
    let tokens = vec![];

    for char in source {
        if char.contains(char) {

        }
    }
}