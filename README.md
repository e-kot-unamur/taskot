# TasKot

> « Tandis qu'ils saccageaient le troupeau, le berger appelait au secours les villageois ; mais ceux-ci, s’imaginant qu’il plaisantait comme d’habitude, se soucièrent peu de lui. »
>
> — L'Enfant qui criait au loup, Ésope
>
> *Cette fois-ci, je ne plaisante plus. Faites vos tâches.*

TasKot est une application qui, tous les lundis à 08h30, envoie à tous les membres d'une colocation, par e-mail, un rappel de la tâche ménagère qu'il doit faire.

## Démarrage

L'application nécessite des variables d'environnement pour configurer l'envoi d'e-mail, et définir les tâches à réaliser ainsi que les membres de la colocation.

Prenons un exemple : Alice, Bob et Claire vivent à trois. Ils définissent trois tâches à accomplir chaque semaine : la vaisselle, la lessive et le nettoyage du sol.

```sh
export EMAIL_HOST='smtp.gmail.com'
export EMAIL_HOST_USERNAME='alice.dupont@gmail.com'
export EMAIL_HOST_PASSWORD='monmotdepassesupersecret'
export EMAIL_FROM='Alice <alice.dupont@gmail.com>'

export TASK_0='Vaisselle'
export TASK_1='Lessive'
export TASK_2='Nettoyage du sol'

export PERSON_0='Alice;alice.dupont@gmail.com'
export PERSON_1='Bob;bob.delamarre@skynet.be'
export PERSON_2='Claire;claire52@gmail.com'

cargo run
```

Une fois démarré, le programme enverra un e-mail à Alice, Bob et Claire tous les lundis à 08h30 en prenant bien soin de faire une tournante chaque semaine.

## Fonctionnalité supplémentaire

Si vous souhaitez en plus lancer une web app qui permet de voir qui doit faire quelle tâche, vous pouvez rajouter cette variable d'environnement avant le `cargo run` :

```sh
export RUN_WEB_SERVER='true'
```

Attention, vous devez avoir configuré NGINX (ou un autre serveur web) pour le bon fonctionnement de cette web app.

Note : La web app est sur le port 8000 par défaut.

Une fois démarrée, vous pourrez y accéder à l'adresse `http://votre-serveur:8000`.
