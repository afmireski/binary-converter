def convert_int_to_binary(number: int) -> str:
    result = ''

    max_value: int = 128
    value: int = number

    while value > 0:
        if value >= max_value:
            result += '1'
            value -= max_value
        else:
            result += '0'
        max_value = max_value//2

    while len(result) < 8:
        result += '0'

    return result


def convert_int_to_binary_excess_127(value: int) -> str:
    if -127 <= value <= 128:
        excess_value: int = value + 127

        excess_binary = convert_int_to_binary(excess_value)

        return excess_binary
    else:
        return '00000000'


def calculate_exponents(int_binary: str, decimal_binary: str) -> (int, str):
    int_exponent: int = 0

    if int_binary.__contains__('1'):
        length: int = len(int_binary)

        last_index = length  # Registra a última posição da string onde um '1' for encontrado
        for i in range(length-1, 0, -1):
            if int_binary[i] == '1':
                last_index = i+1

        int_exponent = last_index

    elif decimal_binary.__contains__('1'):
        length: int = len(decimal_binary)
        j: int = 1

        while decimal_binary[j-1] != '1' and j-1 <= length:
            j += 1

        int_exponent = -j

    binary_exponent = convert_int_to_binary_excess_127(int_exponent)

    return int_exponent, binary_exponent


def convert_decimal_to_binary(value: float) -> str:
    result = ''

    while value != 1.0:
        value = value * 2

        if value > 1.0:
            result += '1'
            value -= 1
        elif value < 1.0:
            result += '0'

    return result


def calculate_mantissa(int_binary: str, float_binary: str, exponent: int) -> str:
    mantissa: str = ''

    if exponent > 0:
        length: int = len(int_binary)

        start = (length - exponent)

        merged_binary = int_binary + float_binary
        i = start
        while len(mantissa) < 23:
            mantissa += merged_binary[i]

            i += 1

            if i == len(merged_binary):
                i = start

    elif exponent < 0:
        start = abs(exponent)

        i = start
        while len(mantissa) < 23:
            mantissa += float_binary[i]

            i += 1

            if i == len(float_binary):
                i = start

    return mantissa


def convert_float_to_binary(number: float) -> str:
    has_signal: int = 0

    if number < 0:
        has_signal = 1
        number = -number

    partioned_number = str(number).split('.')

    int_value = int(partioned_number[0])
    decimal_value = number - int_value

    binary_int_value = convert_int_to_binary(int_value)
    binary_decimal_value = convert_decimal_to_binary(decimal_value)

    exponent_int, exponent_bin = calculate_exponents(binary_int_value, binary_decimal_value)

    mantissa = calculate_mantissa(binary_int_value, binary_decimal_value, exponent_int)

    response = f'{has_signal}{exponent_bin}{mantissa}'

    return response


def main():
    print('Type any float number:')
    number = float(input('> '))

    response = convert_float_to_binary(number)

    print(f'Stored {number}:\n\t[{response}] - {len(response)} bits')



main()