<template>
    <div class="lateral">
        <div class="user">
            <Icon name="account_circle" :size="4" color="var(--bg-secondary)" />
            <p>My account</p>
        </div>
        <div class="qr"></div>
        <div class="calendar">
            <Icon name="calendar_month" :size="4" color="var(--bg-secondary)" />
            <p>My Schedule</p>
        </div>
    </div>
</template>
<style scoped>
.lateral {
    width: 50%;
    background-color: var(--bg-primary);
    position: absolute;
    top: 0;
    right: 0;
    height: 92.539%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
}

.user,
.calendar {
    background-color: var(--color-secondary);
    display: flex;
    margin: 1rem 0;
}

.user p,
.calendar p {
    color: white;
    font-size: 1.2rem;
}
</style>