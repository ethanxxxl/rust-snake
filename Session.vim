let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Documents/Rust/snake
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +1 src/main.rs
badd +13 Cargo.toml
badd +3 ~/.config/nvim/lua/option.lua
badd +45 ~/.config/nvim/lua/lsp.lua
badd +1 ~/.config/nvim/lua/plugins.lua
badd +2 man://self(n)
badd +148 term://~/Documents/Rust/snake//296821:/bin/zsh
badd +0 README.md
argglobal
%argdel
$argadd src/main.rs
set stal=2
tabnew
tabrewind
edit src/main.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
split
1wincmd k
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe '1resize ' . ((&lines * 13 + 15) / 31)
exe '2resize ' . ((&lines * 14 + 15) / 31)
argglobal
balt term://~/Documents/Rust/snake//296821:/bin/zsh
setlocal fdm=syntax
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=7
setlocal fml=1
setlocal fdn=20
setlocal fen
168
normal! zo
169
normal! zo
185
normal! zo
195
normal! zo
195
normal! zc
216
normal! zo
216
normal! zc
251
normal! zo
251
normal! zc
292
normal! zo
329
normal! zo
330
normal! zo
292
normal! zc
345
normal! zo
384
normal! zo
392
normal! zo
409
normal! zo
415
normal! zo
433
normal! zo
434
normal! zo
450
normal! zo
let s:l = 360 - ((4 * winheight(0) + 6) / 13)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 360
normal! 062|
wincmd w
argglobal
if bufexists("term://~/Documents/Rust/snake//296821:/bin/zsh") | buffer term://~/Documents/Rust/snake//296821:/bin/zsh | else | edit term://~/Documents/Rust/snake//296821:/bin/zsh | endif
if &buftype ==# 'terminal'
  silent file term://~/Documents/Rust/snake//296821:/bin/zsh
endif
balt src/main.rs
setlocal fdm=syntax
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=6
setlocal fml=1
setlocal fdn=20
setlocal nofen
let s:l = 57 - ((13 * winheight(0) + 7) / 14)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 57
normal! 026|
wincmd w
2wincmd w
exe '1resize ' . ((&lines * 13 + 15) / 31)
exe '2resize ' . ((&lines * 14 + 15) / 31)
tabnext
edit README.md
argglobal
balt src/main.rs
setlocal fdm=syntax
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=6
setlocal fml=1
setlocal fdn=20
setlocal nofen
let s:l = 2 - ((1 * winheight(0) + 14) / 28)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 2
normal! 0
tabnext 1
set stal=1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0&& getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 shortmess=filnxtToOF
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
let g:this_session = v:this_session
let g:this_obsession = v:this_session
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
