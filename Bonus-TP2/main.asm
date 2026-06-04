BITS 64
org 0x400000
ehdr:
    db 0x7F, "ELF", 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0
    dw 2, 62
    dd 1
    dq _start
    dq phdr - ehdr
    dq 0
    dd 0
    dw 64, 56, 1, 64, 0, 0

phdr:
    dd 1, 5
    dq 0, $$, $$, filesize, filesize, 0x1000

_start:
    mov r12, 0          ; Compteur à 0

.loop_start:
    cmp r12, 10000
    jg .done            ; Si > 10000, on quitte

    mov rax, r12
    mov r8, rsp         ; On sauvegarde le sommet de la pile
    mov rbp, rsp        ; rbp va nous servir de curseur
    
    dec rbp
    mov byte [rbp], 10  ; On pousse le saut de ligne (\n) sur la pile

    mov ebx, 10         ; Diviseur = 10
.itoa_loop:
    xor edx, edx
    div ebx             ; Division
    add dl, '0'         ; Conversion en ASCII
    dec rbp
    mov [rbp], dl       ; On écrit le caractère sur la pile
    test rax, rax
    jnz .itoa_loop      ; Boucle tant qu'il reste des chiffres

    mov rax, 1          ; syscall write
    mov rdi, 1          ; stdout
    mov rsi, rbp        ; adresse de début de notre chaîne sur la pile
    mov rdx, r8
    sub rdx, rbp        ; Calcul de la longueur (fin - début)
    syscall

    inc r12             ; Compteur ++
    jmp .loop_start

.done:
    mov rax, 60         ; syscall exit
    xor rdi, rdi
    syscall

; Constante magique : calcule exactement la taille du fichier !
filesize equ $ - $$