fn main(){
/*  Rust es un lenguaje de programacion que maneja un tipado estatico
    esto quiere decir que tendremos que declarar el tipo de dato de 
    las variables que usaremos asignado un valor de dicho tipo de dato
    ademas de que para los valores de tipo integer o enteros y float en
    rust el lenguaje toma por defecto una longitud de 64 bits y
    si quieres que tenga una longitud menor o mayor se debe especificar
    de esta manera:

    En el caso de integer se utilizan dos letras ya sea "u"  o "i" las
    cuales sigifican esto:              Ademas de una tabla del
    U: Unsigned     I:Signed            numero de bits de tamaño que
                                        puedes usar
    Dicha tabla es la siguiente:
    
    Longitud    signed      Unsingned   en el caso de float solo existen
    8-bit       i8          u8          los 32 y los 64 bits de esta tabla
    16-bit      i16         u16         y por defecto si no especificas el
    32-bit      i32         u32         tamaño y tienes un sistema operativo
    64-bit      i64         u64         de 64 bits rust tomara por defecto
    128-bit     i128        u128        los 64 bits de longitud.
    arch        isize       usize
    
    En el caso de los "u[valor]" pueden representar cualquier espacio de memoria
    ya que son datos con un espacio de memoria no asignado

    Mientras que los valores "i[valor]" representan un espacio ya asignado de memoria
    ademas de que dicho espacio sirve tambien para almacenar arreglos.

    En rust para declarar un tipo de dato boleano hay 2 maneras

    1) se puede declarar como cualquier otra variable asignando el valor asi:
    let variable = true;

    2)Se puede asignar definiedo el tipo de dato boleano (en cuyo caso
    se utiliza la palabra bool) asi:
    let variable: bool = false;

    Tipo caracter:
    En rust si existe el tipo caracter el cual se define colocando un valor de cadena
    a una variable utilizando comillas simplres y colocando un solo caracter asi:
    let caracter = 'A';


    el siguiente codigo muestra como funciona todo lo explicado anteriormente
*/  

    let numero: u32 = 24;
    let decimal: f32 = 3.1416;  //32 bits
    let decimal64 = 8.171609;   // bits
    let condicion : bool = false;
    println!("la variable numero vale: {}, decimal vale: {}, decimal64 vale: {}",numero, decimal, decimal64);
    println!("y condicion tiene asignado: {}",condicion);
}