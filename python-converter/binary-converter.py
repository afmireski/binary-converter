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


def convert_float_to_binary(number: float) -> str:
    has_signal: int = 0
    exponent_bin = ''

    partioned_number = str(number).split('.')

    int_value = int(partioned_number[0])
    float_value = number - int_value

    if number < 0:
        has_signal = 1

    return exponent_bin


def main():
    print('Type any float number:')
    number = int(input('> '))

    # response = convert_float_to_binary(number)

    print(f'Number -> {convert_int_to_binary(number)}')



main()