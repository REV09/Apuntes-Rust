fn main(){
/*  El Shadowing o sombreado por su traduccion literal
    es una herramienta de rust util.
    Te permite declarar nuevas variables con el mismo
    nombre de una variable anterior. Por lo que la
    primer variable sera la sombra de la segunda y la
    segunda sera la sombra de la tercera y asi sucesiva
    mente

    Para acer Shadowing se declara una variable con "let"
    y se le pone el mismo nombre de la anterior, despues
    de eso se le asigna el valor de la anterior variable
    ademas de que este valor se puede modificar.

    Este programa es un ejemplo de como funciona el
    Shadowing

*/
    let variable = 5;
    println!("Variable es igual a {}", variable);
    let variable = variable + 1;
    println!("Variable con Shadowing es igual a: {}", variable);
    let variable = variable * 2;
    println!("Variable ahora vale: {}",variable);
}