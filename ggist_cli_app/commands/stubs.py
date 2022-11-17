import click
from ggist_cli_app.settings import click_pass_settings


@click.command()
@click_pass_settings
def profile_loader(context):
    # apply it in bashrc with 
    # cd /workspaces/ggist/ && eval "$(python -m ggist_cli_app apply)"

    # apply aliases
    with open(settings.aliases_file, 'r') as fin:
        line = fin.read()
        print(line)


@click.command()
@click_pass_settings
def refresh(context):
    settings.sources_manager.save()