FROM mcr.microsoft.com/dotnet/aspnet:{{ dotnet_version }} AS base
#USER $APP_UID
WORKDIR /app
#EXPOSE 8080
#EXPOSE 8081

ENV ASPNETCORE_URLS=http://+:5000

# Set the timezone for the container by setting the TZ environment variable
# to the desired timezone (in this case, America/New_York).
ENV TZ=Asia/Dhaka

# Create a symbolic link from /etc/localtime (used by applications to determine
# the system timezone) to the corresponding timezone file in /usr/share/zoneinfo
# on the host system. This ensures that the container is using the correct timezone.
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && \
\
# Set the timezone in /etc/timezone to ensure that it is used by all applications
# that rely on the timezone configuration.
    echo $TZ > /etc/timezone

# For HealthChecks
RUN apt-get update && apt-get install -y curl

FROM mcr.microsoft.com/dotnet/sdk:{{ dotnet_version }} AS build
ARG BUILD_CONFIGURATION=Release
WORKDIR /src
COPY ["{{ project_directory }}.csproj", "./"]
RUN dotnet restore "{{ project_directory }}.csproj"
COPY . .
WORKDIR "/src/"
RUN dotnet build "{{ project_directory }}.csproj" -c $BUILD_CONFIGURATION -o /app/build

FROM build AS publish
ARG BUILD_CONFIGURATION=Release
RUN dotnet publish "{{ project_directory }}.csproj" -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "{{ project_directory }}.dll"]
