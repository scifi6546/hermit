FROM python
WORKDIR /app
copy . /app
RUN pip3 install -e package/
EXPOSE 8080
CMD ["python","package/app/main.py"]

