FROM cimg/python:3.8

RUN sudo apt-get update -qq \
  && sudo apt-get install -qy --no-install-recommends \
    wine64 \
    gcc-mingw-w64
    
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal \
  && echo 'export PATH=$HOME/.cargo/bin:$PATH' >> ~/.profile

RUN export PATH=$HOME/.cargo/bin:$PATH \
  && rustup target add x86_64-pc-windows-gnu \
  && echo '[target.x86_64-pc-windows-gnu]' >> ~/.cargo/config \
  && echo 'linker = "/usr/bin/x86_64-w64-mingw32-gcc"' >> ~/.cargo/config

RUN wine64-stable wineboot \
  && mkdir ~/winpython \
  && cd ~/winpython \
  && wget https://www.python.org/ftp/python/3.8.2/python-3.8.2-embed-amd64.zip \
  && unzip python-3.8.2-embed-amd64.zip -d ~/winpython \
  && echo "export WINPYTHON=\"wine64-stable $HOME/winpython/python.exe\"" >> ~/.profile \
  && wget https://bootstrap.pypa.io/get-pip.py \
  && wine64-stable ~/winpython/python.exe get-pip.py \
  && echo "import site" >> ~/winpython/python38._pth \
  && echo "import sys; sys.path.insert(0, '')" >> ~/winpython/sitecustomize.py \
  && echo "import ctypes; ctypes.cdll.LoadLibrary('Z:\\\\home\\\\circleci\\\\project\\\\wine_shims\\\\bcryptprimitives.dll')" >> ~/winpython/sitecustomize.py

ADD init.sh /home/circleci/init.sh
ADD build.sh /home/circleci/build.sh

RUN sudo chown circleci:circleci ~/*.sh 

ENTRYPOINT ~/init.sh
