FROM rockylinux:8

LABEL org.opencontainers.image.source="https://github.com/giovtorres/slurm-docker-cluster" \
    org.opencontainers.image.title="slurm-docker-cluster" \
    org.opencontainers.image.description="Slurm Docker cluster on Rocky Linux 8" \
    org.label-schema.docker.cmd="docker-compose up -d" \
    maintainer="Giovanni Torres"

ARG SLURM_TAG=slurm-21-08-6-1
ARG GOSU_VERSION=1.11

RUN set -ex \
    && yum makecache \
    && yum -y update \
    && yum -y install dnf-plugins-core \
    && yum config-manager --set-enabled powertools \
    && yum -y install \
    wget \
    bzip2 \
    perl \
    gcc \
    gcc-c++\
    git \
    gnupg \
    make \
    munge \
    munge-devel \
    python3-devel \
    python3-pip \
    python3 \
    mariadb-server \
    mariadb-devel \
    psmisc \
    bash-completion \
    vim-enhanced \
    http-parser-devel \
    json-c-devel \
    && yum clean all \
    && rm -rf /var/cache/yum

RUN alternatives --set python /usr/bin/python3

RUN pip3 install Cython nose

RUN set -ex \
    && wget -O /usr/local/bin/gosu "https://github.com/tianon/gosu/releases/download/$GOSU_VERSION/gosu-amd64" \
    && wget -O /usr/local/bin/gosu.asc "https://github.com/tianon/gosu/releases/download/$GOSU_VERSION/gosu-amd64.asc" \
    && export GNUPGHOME="$(mktemp -d)" \
    && gpg --batch --keyserver hkps://keys.openpgp.org --recv-keys B42F6819007F00F88E364FD4036A9C25BF357DD4 \
    && gpg --batch --verify /usr/local/bin/gosu.asc /usr/local/bin/gosu \
    && rm -rf "${GNUPGHOME}" /usr/local/bin/gosu.asc \
    && chmod +x /usr/local/bin/gosu \
    && gosu nobody true

RUN set -x \
    && git clone -b ${SLURM_TAG} --single-branch --depth=1 https://github.com/SchedMD/slurm.git \
    && pushd slurm \
    && ./configure --enable-debug --prefix=/usr --sysconfdir=/etc/slurm \
    --with-mysql_config=/usr/bin  --libdir=/usr/lib64 \
    && make install \
    && install -D -m644 etc/cgroup.conf.example /etc/slurm/cgroup.conf.example \
    && install -D -m644 etc/slurm.conf.example /etc/slurm/slurm.conf.example \
    && install -D -m644 etc/slurmdbd.conf.example /etc/slurm/slurmdbd.conf.example \
    && install -D -m644 contribs/slurm_completion_help/slurm_completion.sh /etc/profile.d/slurm_completion.sh \
    && popd \
    && rm -rf slurm \
    && groupadd -r --gid=990 slurm \
    && useradd -r -g slurm --uid=990 slurm \
    && mkdir /etc/sysconfig/slurm \
    /var/spool/slurmd \
    /var/run/slurmd \
    /var/run/slurmdbd \
    /var/lib/slurmd \
    /var/log/slurm \
    /data \
    && touch /var/lib/slurmd/node_state \
    /var/lib/slurmd/front_end_state \
    /var/lib/slurmd/job_state \
    /var/lib/slurmd/resv_state \
    /var/lib/slurmd/trigger_state \
    /var/lib/slurmd/assoc_mgr_state \
    /var/lib/slurmd/assoc_usage \
    /var/lib/slurmd/qos_usage \
    /var/lib/slurmd/fed_mgr_state \
    && chown -R slurm:slurm /var/*/slurm* \
    && /sbin/create-munge-key

COPY slurm.conf /etc/slurm/slurm.conf
COPY slurmdbd.conf /etc/slurm/slurmdbd.conf
RUN set -x \
    && chown slurm:slurm /etc/slurm/slurmdbd.conf \
    && chmod 600 /etc/slurm/slurmdbd.conf

# Enable access via ssh from the app
RUN dnf install openssh-server -y
# Copy already created ssh rsa host keys 
# to ensure the known host key file always get the same fingerprint
COPY ssh_host_rsa_key /etc/ssh/ssh_host_rsa_key 
COPY ssh_host_rsa_key.pub /etc/ssh/ssh_host_rsa_key.pub
RUN chmod 600 /etc/ssh/ssh_host_rsa_key /etc/ssh/ssh_host_rsa_key.pub

# This user is used for ssh connection and slurm operations.
RUN adduser dev_user 
RUN echo "dev_user:password" | chpasswd
RUN echo "dev_user ALL = (root) NOPASSWD: ALL" >> /etc/sudoers
# Needed to allow to log in as dev_user as unprivileged user 
RUN rm -rf /run/nologin

# Providing local ssh access for user dev_user. 
# Meant to allow authentication via ssh-agent.
RUN mkdir /home/dev_user/.ssh && chown dev_user /home/dev_user/.ssh && chmod 0700 /home/dev_user/.ssh
RUN touch /home/dev_user/.ssh/authorized_keys && chown dev_user /home/dev_user/.ssh/authorized_keys && chmod 0600 /home/dev_user/.ssh/authorized_keys
COPY dev_user_ed25519.pub /home/dev_user/dev_user_ed25519.pub
RUN cat /home/dev_user/dev_user_ed25519.pub >> /home/dev_user/.ssh/authorized_keys && rm /home/dev_user/dev_user_ed25519.pub

COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
# Copied for providing script which is executed to provide initial specs and data. 
# It is invoked by the docker-entrypoint script at runtime
COPY add_slurm_data.sh /usr/local/bin/add_slurm_data.sh
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]

CMD ["slurmdbd"]
