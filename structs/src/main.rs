fn main() {
    let conta = Conta {
        titular: Titular {
            nome: String::from("Ryan"),
            sobrenome: String::from("Pereira"),
        },
        saldo: 1.5,
    };

    println!("{} possui R${}", conta.titular.fullname(), conta.saldo);
}

struct Conta {
    titular: Titular,
    saldo: f32,
}

struct Titular {
    nome: String,
    sobrenome: String,
}

impl Titular {
    fn fullname(&self) -> String {
        format!("{} {}", self.nome, self.sobrenome)
    }
}
