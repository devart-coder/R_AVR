#!/bin/bash
# Создаем массив для новых аргументов
NEW_ARGS=()

for arg in "$@"; do
    # Удаляем подстроки, которые ломают avr-ld, даже если они внутри -Wl,...
    temp_arg=$(echo "$arg" | sed 's/--eh-frame-hdr//g' | sed 's/-z,noexecstack//g' | sed 's/--as-needed//g')
    
    # Если после удаления остались пустые запятые (например, -Wl,,), убираем их
    temp_arg=$(echo "$temp_arg" | sed 's/,,/,/g' | sed 's/,-Wl//g' | sed 's/-Wl,$//g')
    
    # Если аргумент не стал пустым, добавляем его
    if [ "$temp_arg" != "-Wl" ] && [ ! -z "$temp_arg" ]; then
        NEW_ARGS+=("$temp_arg")
    fi
done

# Вызываем реальный avr-gcc с очищенными аргументами
exec avr-gcc "${NEW_ARGS[@]}"

