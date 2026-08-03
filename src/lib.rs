pub fn validar_cpf(cpf: &str) -> bool {
    // Remove caracteres não numéricos (pontos e traço)
    let digitos: Vec<u32> = cpf.chars().filter_map(|c| c.to_digit(10)).collect();

    // CPF deve ter exatamente 11 dígitos
    if digitos.len() != 11 {
        return false;
    }

    // Rejeita CPFs com todos os dígitos iguais (ex: 111.111.111-11)
    if digitos.iter().all(|&d| d == digitos[0]) {
        return false;
    }

    // Calcula o primeiro dígito verificador
    let primeiro_dv = calcular_digito_verificador(&digitos[0..9]);
    if primeiro_dv != digitos[9] {
        return false;
    }

    // Calcula o segundo dígito verificador
    let segundo_dv = calcular_digito_verificador(&digitos[0..10]);
    if segundo_dv != digitos[10] {
        return false;
    }

    true
}

fn calcular_digito_verificador(digitos: &[u32]) -> u32 {
    let peso_inicial = digitos.len() as u32 + 1;

    let soma: u32 = digitos
        .iter()
        .enumerate()
        .map(|(i, &d)| d * (peso_inicial - i as u32))
        .sum();

    let resto = soma % 11;

    if resto < 2 { 0 } else { 11 - resto }
}
