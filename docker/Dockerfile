FROM ubuntu:20.04

ENV TZ=Asia/Tokyo
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends tzdata git-all tmux nano curl build-essential libffi-dev libssl-dev zlib1g-dev liblzma-dev libbz2-dev \
    libreadline-dev libsqlite3-dev libopencv-dev tk-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY ./docker /root/docker

# pyenv
RUN curl https://pyenv.run | bash
RUN echo '. /root/docker/pyenv_setting.sh' >> /root/.bashrc

RUN . /root/docker/pyenv_setting.sh && \
    pyenv install 3.10.13 && \
    pyenv global 3.10.13 && \
    curl -sSL https://install.python-poetry.org | python -

# poetry
RUN echo '. /root/docker/poetry_setting.sh' >> /root/.bashrc

CMD ["/bin/bash"]