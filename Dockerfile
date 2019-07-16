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
RUN mkdir ~/app/thumbnails && pip3 install --user -e package/
EXPOSE 8080
CMD ["python","package/app/main.py"]

