# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" width="256" height="256" />
</p>

<p align="center">~ Una transferencia de archivos de terminal rica en funciones ~</p>
<p align="center">
  <a href="https://veeso.github.io/termscp/" target="_blank">Sitio Web</a>
  ·
  <a href="https://veeso.github.io/termscp/#get-started" target="_blank">Instalación</a>
  ·
  <a href="https://veeso.github.io/termscp/#user-manual" target="_blank">Manual de usuario</a>
</p>

<p align="center">
  <a href="https://github.com/veeso/termscp"
    ><img
      height="20"
      src="/assets/images/flags/us.png"
      alt="English"
  /></a>
  &nbsp;
  <a
    href="/docs/de/README.md"
    ><img
      height="20"
      src="/assets/images/flags/de.png"
      alt="Deutsch"
  /></a>
  &nbsp;
  <a
    href="/docs/es/README.md"
    ><img
      height="20"
      src="/assets/images/flags/es.png"
      alt="Español"
  /></a>
  &nbsp;
  <a
    href="/docs/fr/README.md"
    ><img
      height="20"
      src="/assets/images/flags/fr.png"
      alt="Français"
  /></a>
  &nbsp;
  <a
    href="/docs/it/README.md"
    ><img
      height="20"
      src="/assets/images/flags/it.png"
      alt="Italiano"
  /></a>
  &nbsp;
  <a
    href="/docs/zh-CN/README.md"
    ><img
      height="20"
      src="/assets/images/flags/cn.png"
      alt="简体中文"
  /></a>
</p>

<p align="center">Desarrollado por <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Versión actual: 0.7.0 (12/10/2021)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/termscp/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/termscp.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/termscp"
    ><img
      src="https://img.shields.io/crates/d/termscp.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/termscp"
    ><img
      src="https://img.shields.io/crates/v/termscp.svg"
      alt="Latest version"
  /></a>
  <a href="https://www.buymeacoffee.com/veeso"
    ><img
      src="https://img.shields.io/badge/Donate-BuyMeACoffee-yellow.svg"
      alt="Buy me a coffee"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/Linux/badge.svg"
      alt="Linux CI"
  /></a>
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/MacOS/badge.svg"
      alt="MacOS CI"
  /></a>
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/Windows/badge.svg"
      alt="Windows CI"
  /></a>
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/FreeBSD/badge.svg"
      alt="FreeBSD CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/termscp"
    ><img
      src="https://coveralls.io/repos/github/veeso/termscp/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/termscp"
    ><img
      src="https://docs.rs/termscp/badge.svg"
      alt="Docs"
  /></a>
</p>

---

## Sobre termscp 🖥

Termscp es un explorador y transferencia de archivos de terminal rico en funciones, con apoyo para SCP/SFTP/FTP/S3. Básicamente, es una utilidad de terminal con una TUI para conectarse a un servidor remoto para recuperar y cargar archivos e interactuar con el sistema de archivos local. Es compatible con **Linux**, **MacOS**, **FreeBSD** y **Windows**.

![Explorer](/assets/images/explorer.gif)

---

## Características 🎁

- 📁  Diferentes protocolos de comunicación
  - **SFTP**
  - **SCP**
  - **FTP** y **FTPS**
  - **Aws S3**
- 🖥  Explore y opere en el sistema de archivos de la máquina local y remota con una interfaz de usuario práctica
  - Cree, elimine, cambie el nombre, busque, vea y edite archivos
- ⭐  Conéctese a sus hosts favoritos y conexiones recientes
- 📝  Ver y editar archivos con sus aplicaciones favoritas
- 💁  Autenticación SFTP / SCP con claves SSH y nombre de usuario / contraseña
- 🐧  compatible con Linux, MacOS, FreeBSD y Windows
- 🎨  Haz lo tuyo!
  - Temas
  - Formato de explorador de archivos personalizado
  - Editor de texto personalizable
  - Clasificación de archivos personalizable
  - y muchos otros parámetros ...
- 📫  Reciba una notificación cuando se haya transferido un archivo grande
- 🔐  Guarde su contraseña en el almacén de claves de su sistema operativo
- 🦀  Rust-powered
- 👀  Desarrollado sin perder de vista el rendimiento
- 🦄  Actualizaciones frecuentes

---

## Para iniciar 🚀

Si estás considerando instalar termscp, ¡quiero darte las gracias 💜! ¡Espero que disfrutes de termscp!
Si desea contribuir a este proyecto, no olvide consultar nuestra [guía de contribución](../../CONTRIBUTING.md).

Si tu eres un usuario de Linux, FreeBSD o MacOS, este sencillo script de shell instalará termscp en tu sistema con un solo comando:

```sh
curl --proto '=https' --tlsv1.2 -sSLf "https://git.io/JBhDb" | sh
```

mientras que si eres un usuario de Windows, puedes instalar termscp con [Chocolatey](https://chocolatey.org/):

```sh
choco install termscp
```

Para obtener más información u otras plataformas, visite [veeso.github.io](https://veeso.github.io/termscp/#get-started) para ver todos los métodos de instalación.

⚠️ Si estás buscando cómo actualizar termscp, simplemente ejecute termscp desde CLI con:: `(sudo) termscp --update` ⚠️

### Requisitos ❗

- Usuarios **Linux**:
  - libssh
  - libdbus-1
  - pkg-config
- Usuarios **FreeBSD**:
  - libssh
  - dbus
  - pkgconf

### Requisitos opcionales ✔️

These requirements are not forced required to run termscp, but to enjoy all of its features

- Usuarios **Linux/FreeBSD**:
  - Para **abrir** archivos con `V` (al menos uno de estos)
    - *xdg-open*
    - *gio*
    - *gnome-open*
    - *kde-open*
- Usuarios **Linux**:
  - Un keyring manager: leer más en el [manual de usuario](man.md#linux-keyring)
- Usuarios **WSL**
  - Para **abrir** archivos con `V` (al menos uno de estos)
    - [wslu](https://github.com/wslutilities/wslu)

---

## Apoyame ☕

Si te gusta termscp y te encantaría que el proyecto crezca y mejore, considera una pequeña donación para apoyarme 🥳

Puedes hacer una donación con una de estas plataformas:

[![Buy-me-a-coffee](https://img.shields.io/badge/Donate-BuyMeACoffee-yellow.svg)](https://www.buymeacoffee.com/veeso)
[![PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg)](https://www.paypal.me/chrisintin)

---

## Manual de usuario y documentación 📚

El manual del usuario se puede encontrar en el [sitio web de termscp](https://veeso.github.io/termscp/#user-manual) o en [Github](man.md).

La documentación para desarrolladores se puede encontrar en Rust Docs en <https://docs.rs/termscp>

---

## Problemas conocidos 🧻

- `NoSuchFileOrDirectory` al conectar (WSL1): Conozco este problema y supongo que es un problema técnico de WSL. No se preocupe, simplemente mueva el ejecutable termscp a otra ubicación en el PATH, como `/usr/bin`, o instálelo a través del formato de paquete apropiado (por ejemplo, deb).

---

## Contribuir y problemas 🤝🏻

¡Las contribuciones, los informes de errores, las nuevas funciones y las preguntas son bienvenidas! 😉
Si tiene alguna pregunta o inquietud, o si desea sugerir una nueva función, o simplemente desea mejorar termscp, no dude en abrir un problema o un PR.

Sigue [nuestras pautas de contribución](../../CONTRIBUTING.md)

---

## Changelog ⏳

Ver registro de cambios de termscp [AQUÍ](../../CHANGELOG.md)

---

## Powered by 💪

termscp funciona con estos increíbles proyectos:

- [bytesize](https://github.com/hyunsik/bytesize)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [edit](https://github.com/milkey-mouse/edit)
- [keyring-rs](https://github.com/hwchen/keyring-rs)
- [open-rs](https://github.com/Byron/open-rs)
- [rpassword](https://github.com/conradkleinespel/rpassword)
- [rust-s3](https://github.com/durch/rust-s3)
- [self_update](https://github.com/jaemk/self_update)
- [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)
- [suppaftp](https://github.com/veeso/suppaftp)
- [textwrap](https://github.com/mgeisler/textwrap)
- [tui-rs](https://github.com/fdehau/tui-rs)
- [tui-realm](https://github.com/veeso/tui-realm)
- [whoami](https://github.com/libcala/whoami)
- [wildmatch](https://github.com/becheran/wildmatch)

---

## Galería 🎬

> Termscp Home

![Auth](/assets/images/auth.gif)

> Bookmarks

![Bookmarks](/assets/images/bookmarks.gif)

> Setup

![Setup](/assets/images/config.gif)

> Text editor

![TextEditor](/assets/images/text-editor.gif)

---

## Licencia 📃

termscp tiene la licencia MIT.

Puede leer la licencia completa [AQUÍ](../../LICENSE)