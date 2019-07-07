FROM python
WORKDIR /app
copy . /app
#run apt install python3.7
#RUN python3 -m venv venv
#RUN source venv/bin/activate
RUN pip3 install -e package/
EXPOSE 8080
CMD ["python","package/app/main.py"]

