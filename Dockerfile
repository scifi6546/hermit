FROM python
run apt update && apt install ffmpegthumbnailer -y
RUN groupadd -g 999 appuser && \
    useradd -r -u 999 -g appuser appuser
WORKDIR /home/appuser/app
RUN chown appuser:appuser /home/appuser
RUN chown appuser /home/appuser/app
USER appuser
COPY --chown=999 . /home/appuser/app
RUN ls -al ~/
RUN ls -al ~/app
RUN export PATH="/home/appuser/.local/bin:$PATH" && mkdir ~/app/thumbnails && pip3 install --user -e package/
RUN ls /home/appuser/.local/bin
EXPOSE 8080
CMD ["/home/appuser/.local/bin/gunicorn","app:app","-b","0.0.0.0:8080"]

#CMD ["python","package/app/main.py"]

