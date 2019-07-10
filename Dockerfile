FROM python
WORKDIR /app
copy . /app
RUN pip3 install -e package/
run apt update && apt install ffmpegthumbnailer -y
EXPOSE 8080
CMD ["python","package/app/main.py"]

