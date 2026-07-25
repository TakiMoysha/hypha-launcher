## Architecture - Feature Sliced Design


## Issues

- in vue style tag not working `apply` from unocss, added `./src/assets/css/fromuno.css`;

## Features

- [ProcessKit for controll spawn process](https://zelanton.github.io/processkit/)

#### Launcher Extender

Либа, которая инжектится в игру при старте и добавляет overlay, где можно посмотрть моды и информацию по ним. Отображает логи запущенного сервера или инфу из лаунчера (общение с лаунчером через сокет).

#### Running Timer

Зависит от "супервизора" процесса (server/client). Добавлять ли launcher extender - overlay для игры с параметрами отладки.

## TASKS

- [ ] added mock server for plugins (testing, tracers bullets, etc.)

# Hypha Runner

Запускает сервак и управляет им:
- docker/podman
- process

- namespaces (CLONE_NEWUSER): isolated resources from host + u/g-id_mapping
- root pivot: fake FS (`chroot` and `pivot_root`, second have security measure and restrict root access)
- cgroups v2: `/sys/fs/cgroup/` and within it are multiple views into the kernel. `memory.max`, `pids.max`
- overlayfs: lower (base image, nothing is written) & upper (all the writes take place) parts
- capabilities
- seccomp
- landlock

> `PR_SET_NO_NEW_PRIVS` - запрет на получение привилегий для дочек
> `PR_SET_PDEATHSIG` - привязать дочку на родителя (от зомбей)

```
[ Хост (Rust Runner) ]
  ├── 1. OverlayFS (Upper/Lower/Merged)
  ├── 2. Cgroups v2 (limits RAM/CPU)
  └── 3. fork() -> unshare(NEWNS | NEWPID | NEWNET | NEWIPC)
      └── game-server *ARGS
         ├── pivot_root() (Закрытая файловая система)
         ├── mount tmpfs on /dev (+ mknod null, tty, urandom)
         ├── mount proc on /proc
         ├── setuid(1001) / setgid(1001) (Сброс root)
         ├── caps::clear() (Сброс всех Linux Capabilities)
         └── execve("java") -> [ game-server ]
```


##### Unprivileged Runner

Запрещает вызов `mknod` (EPERM), создание символьных/блочных устройств ограничено 


## References

- [просмотрщик логов / dlt-tui](https://github.com/tkmsikd/dlt-tui)

