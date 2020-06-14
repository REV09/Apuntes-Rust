/* Los tipos de datos compuestos en rust que existen son las tuplas, y los arreglos
o tambien conocidos como vectores(Hay otro apunte que se enfoca exclusivamente a los vectores).
(Este apunte solo esta enfocado a las tuplas, si eres nuevo programando te recomiendo que no te
saltes este apunte.) 

 
Para declarar una tupla en rust, se realiza
    de una manera similar que una variable con
    la diferencia de que sus valores se van a
    encapsular en un parentesis y cada uno de 
    sus valores seran separados por una coma
    "," cada uno de estos valores tiene un tipo
    y cada tipo cada uno de los tipos de estos 
    valores no deben ser los mismo. Ejemplo:

    let tupla: (i8, u16, f32 ) = (6, 28, 8,9);

    la variable tupla cuenta como todos estos
    valores juntos ya que se considera como un
    compuesto singular de todos estos elementos,
    si queremos que tener acceso a cada uno de
    estos valores individuales y poder hacer
    uso de ellos de manera individual, hay 
    que almacenar cada valor en una varible,
    una manera de hacer esto sin usar muchas 
    lineas de codigo es con los patrones de 
    asignacion de esta manera:

    let (v1, v2, v3) = tupla;

    con esa linea de comando solo bastaria hacer
    un println! para mostrar el elemento que 
    gustes en pantalla.

    Por otro lado si lo unico que quieres es
    mostrar dicho dato en pantalla solo debes usar
    un println! de esta manera:

    (Recordemos que en un arreglos, tuplas, listas,
    etc. Las posiciones de los valores empiezan desde
    cero por lo que si quieres mostrar el segundo
    valor almacenado debes de leer la posicion uno.)

    println!("{}", tupla.[posicion del valor])
        Notese que se colocan llaves entre comillas
        esto se hace porque si quisieramos imprimir
        directamente en pantalla el valor de la tupla
        obtendremos el siguiente error.

        error: format argument must be a string literal
  --> TiposDatosCompuestos.rs:52:14
   |
52 |     println!(tupla.0);
   |              ^^^^^^^
   |
help: you might be missing a string literal to format with
   |
52 |     println!("{}", tupla.0);
   |              ^^^^^

error: aborting due to previous error

        Este error es por que el compilador de rust
        nos pide que formateemos la variable de manera
        que se pueda leer como un string

    
*/
 
fn main(){
    let tupla: (i64, i32, f32) = (12, 32, 8.65);
    println!("Impresion del valor de tupla en la posicion 1 (la posicion cero '0') sin usar variables {}", tupla.0);
    let (v1, v2, v3) = tupla;
    println!("Impresion del valor de tupla en la posicion 2 (la posicion '1') usando una variable {}", v2);
}

/*  NOTA FINAL:
    Al compilar este codigo en linux nos apareceran un par
    de warnings sugiriendonos usar declarar mejor v1, v2 y v3
    con un guion bajo asi: "_v1" 
*/